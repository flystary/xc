use anyhow::Result;
use openssh::{KnownHosts, SessionBuilder};
use std::env;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<()> {
    // ---------- 获取命令行参数 ----------
    let args: Vec<String> = env::args().collect();
    if args.len() != 7 {
        eprintln!(
            "Usage: {} <node_ip> <cpe_ip> <username> <password> <mode> <command>",
            args[0]
        );
        std::process::exit(1);
    }

    // 克隆 String，避免 &String 类型错误
    let node_ip = args[1].clone();
    let cpe_ip = args[2].clone();
    let username = args[3].clone();
    let password = args[4].clone();
    let mode = args[5].clone();
    let command = args[6].clone();

    let x_pwd = "ROOTPWD"; // root 密码

    // ---------- 1. 登录跳板机 ----------
    let jump_session = SessionBuilder::default()
        .user(username.clone())
        .password(password.clone()) // 低版本 openssh 支持 password()
        .known_hosts_check(KnownHosts::Accept)
        .connect("master.jump.7x-networks.net:2222")
        .await?;

    println!("✅ Logged into jump host");

    // ---------- 2. 切换节点 root ----------
    let mut jump_shell = jump_session.shell().await?;
    jump_shell
        .write_all(format!("{}\n", node_ip).as_bytes())
        .await?;
    jump_shell.write_all(b"su -\n").await?;
    jump_shell
        .write_all(format!("{}\n", x_pwd).as_bytes())
        .await?;
    jump_shell.flush().await?;
    println!("✅ Switched to node root");

    // ---------- 3. SSH 到目标 CPE ----------
    let cpe_user = if mode == "ucpe" {
        "root".to_string()
    } else {
        username.clone()
    };

    let mut cpe_session = jump_session
        .command("ssh")
        .arg("-i")
        .arg("/etc/openvpn/server/box")
        .arg(format!("{}@{}", cpe_user, cpe_ip))
        .arg("-p")
        .arg("7722")
        .stdin_open()
        .stdout_open()
        .stderr_open()
        .spawn()
        .await?;

    // 普通用户模式切换 root
    if mode != "ucpe" {
        if let Some(mut stdin) = cpe_session.stdin().take() {
            stdin.write_all(b"su -\n").await?;
            stdin.write_all(format!("{}\n", x_pwd).as_bytes()).await?;
            stdin.flush().await?;
        }
    }

    // ---------- 4. 执行动态命令 ----------
    if let Some(mut stdin) = cpe_session.stdin().take() {
        stdin.write_all(format!("{}\n", command).as_bytes()).await?;
        stdin.flush().await?;
    }

    // ---------- 5. 交互模式 ----------
    println!("✅ Entering interactive mode. Press Ctrl+D to exit.");

    let mut stdin = cpe_session.stdin().take().unwrap();
    let mut stdout = cpe_session.stdout().take().unwrap();
    let mut stderr = cpe_session.stderr().take().unwrap();

    let stdin_to_remote = async {
        let _ = io::copy(&mut io::stdin(), &mut stdin).await?;
        Ok::<(), anyhow::Error>(())
    };

    let remote_to_stdout = async {
        let mut combined = io::stdout();
        tokio::select! {
            res1 = io::copy(&mut stdout, &mut combined) => { let _ = res1?; },
            res2 = io::copy(&mut stderr, &mut combined) => { let _ = res2?; },
        }
        Ok::<(), anyhow::Error>(())
    };

    tokio::try_join!(stdin_to_remote, remote_to_stdout)?;

    Ok(())
}
