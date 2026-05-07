#include <expect.h>
#include <iostream>
#include <string>
#include <vector>
#include <tuple>
#include <thread>
#include <mutex>
#include <queue>
#include <condition_variable>
#include <cstdlib>

// ----------------- 会话结构体 -----------------
struct Session
{
    int spawn_id;
    std::string node_ip;
    std::string cpe_ip;
    std::queue<std::string> cmd_queue;
    std::mutex mtx;
    std::condition_variable cv;
    bool active;
    std::thread thread;

    Session(const std::string &n, const std::string &c)
        : spawn_id(0), node_ip(n), cpe_ip(c), active(true) {}
};

// ----------------- 工具函数 -----------------
void send_and_expect(int spawn_id, const std::string &send_str, const std::vector<std::string> &expects)
{
    exp_send(spawn_id, (char *)send_str.c_str());
    for (size_t i = 0; i < expects.size(); ++i)
    {
        exp_expectl(spawn_id,
                    exp_glob, expects[i].c_str(),
                    exp_end);
    }
}

// ----------------- 登录跳板机 -----------------
bool login_jump(Session *session, const std::string &username,
                const std::string &password, const std::string &root_pwd)
{
    session->spawn_id = exp_spawnl("/usr/bin/ssh", "ssh", (char *)username.c_str(),
                                   ("@master.jump.7x-networks.net").c_str(), "-p", "2222", (char *)NULL);
    if (session->spawn_id < 0)
    {
        std::cerr << "Failed to spawn SSH for " << session->cpe_ip << "\n";
        return false;
    }
    send_and_expect(session->spawn_id, "", {"yes/no", "*assword:"});
    send_and_expect(session->spawn_id, session->node_ip + "\n", {"Opt>"});
    send_and_expect(session->spawn_id, "su -\n", {"seven@*~]$"});
    send_and_expect(session->spawn_id, root_pwd + "\n", {"*assword:", "root@*~]#"});
    return true;
}

// ----------------- SSH 到 CPE -----------------
bool ssh_to_cpe(Session *session, const std::string &mode, const std::string &root_pwd, const std::string &o_pwd)
{
    char ssh_cmd[512];
    if (mode != "ucpe")
    {
        snprintf(ssh_cmd, sizeof(ssh_cmd), "ssh -i /etc/openvpn/server/box seven@%s -p 7722\n", session->cpe_ip.c_str());
    }
    else
    {
        snprintf(ssh_cmd, sizeof(ssh_cmd), "ssh -i /etc/openvpn/server/box root@%s -p 7722\n", session->cpe_ip.c_str());
    }
    exp_send(session->spawn_id, ssh_cmd);
    send_and_expect(session->spawn_id, "", {"yes/no", "Enter passphrase for key '/etc/openvpn/server/box':"});
    send_and_expect(session->spawn_id, o_pwd + "\n", {"$"});

    if (mode != "ucpe")
    {
        send_and_expect(session->spawn_id, "su -\n", {"$"});
        send_and_expect(session->spawn_id, root_pwd + "\n", {"#"});
    }
    return true;
}

// ----------------- 会话线程函数 -----------------
void session_thread(Session *session,
                    const std::string &username,
                    const std::string &password,
                    const std::string &mode,
                    const std::string &root_pwd,
                    const std::string &o_pwd)
{
    if (!login_jump(session, username, password, root_pwd))
        return;
    if (!ssh_to_cpe(session, mode, root_pwd, o_pwd))
        return;

    while (session->active)
    {
        std::unique_lock<std::mutex> lock(session->mtx);
        session->cv.wait(lock, [session]
                         { return !session->cmd_queue.empty() || !session->active; });
        while (!session->cmd_queue.empty())
        {
            std::string cmd = session->cmd_queue.front();
            session->cmd_queue.pop();
            lock.unlock();

            // 执行命令
            exp_send(session->spawn_id, (char *)cmd.c_str());
            exp_send(session->spawn_id, "\n");
            std::cout << "[" << session->node_ip << "->" << session->cpe_ip << "] " << cmd << "\n";

            lock.lock();
        }
    }
    exp_close(session->spawn_id);
}

// ----------------- 主函数 -----------------
int main()
{
    const char *username = std::getenv("SSH_USER");
    const char *password = std::getenv("SSH_PASSWORD");
    const char *root_pwd = std::getenv("ROOT_PASSWORD");
    const char *o_pwd = std::getenv("KEY_PASSPHRASE");

    if (!username || !password || !root_pwd || !o_pwd)
    {
        std::cerr << "Please set SSH_USER, SSH_PASSWORD, ROOT_PASSWORD, KEY_PASSPHRASE environment variables\n";
        return 1;
    }

    std::string mode, command;
    std::cout << "Mode (ucpe/other): ";
    std::getline(std::cin, mode);
    std::cout << "Command to execute initially (can send later): ";
    std::getline(std::cin, command);

    // 批量节点示例
    std::vector<std::tuple<std::string, std::string>> nodes;
    nodes.push_back(std::make_tuple("192.168.1.10", "10.0.0.5"));
    nodes.push_back(std::make_tuple("192.168.1.11", "10.0.0.6"));
    nodes.push_back(std::make_tuple("192.168.1.12", "10.0.0.7"));

    // 创建会话
    std::vector<Session *> sessions;
    for (size_t i = 0; i < nodes.size(); ++i)
    {
        std::string nip = std::get<0>(nodes[i]);
        std::string cip = std::get<1>(nodes[i]);
        Session *s = new Session(nip, cip);
        sessions.push_back(s);
        s->thread = std::thread(session_thread, s,
                                username, password, mode, root_pwd, o_pwd);
        if (!command.empty())
        {
            std::lock_guard<std::mutex> lock(s->mtx);
            s->cmd_queue.push(command);
            s->cv.notify_one();
        }
    }

    // 主线程命令管理
    size_t current = 0;
    std::string line;
    while (true)
    {
        std::cout << "cmd> ";
        std::getline(std::cin, line);
        if (line == "exit")
            break;
        else if (line.find("switch") == 0)
        {
            size_t idx = std::stoi(line.substr(7));
            if (idx < sessions.size())
                current = idx;
            else
                std::cout << "Invalid session index\n";
        }
        else if (line == "list")
        {
            for (size_t i = 0; i < sessions.size(); ++i)
                std::cout << i << ": " << sessions[i]->node_ip << "->" << sessions[i]->cpe_ip << "\n";
        }
        else
        {
            std::lock_guard<std::mutex> lock(sessions[current]->mtx);
            sessions[current]->cmd_queue.push(line);
            sessions[current]->cv.notify_one();
        }
    }

    // 关闭所有会话
    for (size_t i = 0; i < sessions.size(); ++i)
    {
        {
            std::lock_guard<std::mutex> lock(sessions[i]->mtx);
            sessions[i]->active = false;
            sessions[i]->cv.notify_one();
        }
        if (sessions[i]->thread.joinable())
            sessions[i]->thread.join();
        delete sessions[i];
    }

    return 0;
}
