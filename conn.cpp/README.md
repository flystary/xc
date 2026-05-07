# Multi-Session SSH Tool (C++11 + libexpect)

## 功能

- 支持跳板机 → 节点 → CPE 多级 SSH
- 多线程并行登录多台 CPE
- 主线程支持会话切换、发送命令、列出会话
- 每条命令输出带 `[node_ip->cpe_ip]` 前缀
- 密码、root 密码、私钥口令通过环境变量安全获取

## 安装依赖

### Ubuntu / Debian
```bash
sudo apt update
sudo apt install g++ cmake libexpect-dev tcl-dev

## 编译
mkdir build
cd build
cmake ..
make

## 环境变量

请在运行前设置以下环境变量：

```bash
export SSH_USER=seven
export SSH_PASSWORD=your_jump_password
export ROOT_PASSWORD=your_root_password
export KEY_PASSPHRASE=your_private_key_passphrase
