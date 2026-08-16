```bash

# xc (Advanced SD-WAN CLI)
xc 是一个基于 Rust 构建的工业级、跨平台命令行工具，专为 SD-WAN 复杂场景下的设备运维而生。它在架构上深度融合了 泛型数据模型、单例模式 与 多协程异步处理 技术，并无缝支持 JumpServer + 多层级跳板机 接入, 支持在 Windows 和 macOS 环境下提供一致的极致运维体验。

## 🏗 核心架构设计
### 1. 泛型与接口驱动的复用模型
为了实现代码的高度复用，xc 放弃了冗余的重复定义，转而采用 Rust 泛型 (Generics)：
- 统一抽象：利用 [T: Info] 泛型约束和接口，定义了 CPE、Dev、POP 等实体的通用数据结构。
- 复用逻辑：序列化、内存索引构建以及表格渲染逻辑均基于泛型实现，确保了不同设备类型处理逻辑的高度一致性。

### 2. 设计模式的应用
- 单例模式 (Singleton)：
  - Token 鉴权中心：确保全局唯一 Token 刷新实例，防止高并发下 OSS 接口调用冲突及频率限制。
  - OSS Route 配置：全局共享统一的路由分发配置，保证跨平台环境下的配置同步。

### 3. 高并发加速引擎
- 协程并发池：启动 3*5 个 Goroutine 竞争解析来自 SDWAN OSS 的原始数据，将海量节点的 Token 获取与数据初始化耗时缩减至秒级。

## 💻 跨平台支持 (Multi-Platform)
`xc` 经过严格测试，在不同操作系统下表现卓越：
*   **Linux (CentOS)**：原生支持 bash/zsh，提供稳定的终端交互体验，并针对常见的 Linux 发行版进行了兼容性优化。
*   **macOS (Darwin)**：完美适配 zsh/iterm2，支持彩色终端输出及 SSH 自动调起。
*   **Windows**：适配 PowerShell 与 CMD，通过针对性的终端编码处理，确保 tablewriter 渲染的彩色表格不乱码。

## 🛠 技术特性
- 内存级多维索引：将数据常驻内存，支持通过 SN、POP 地址、企业信息 进行秒级检索。
- 可视化呈现：基于 tabled 提供直观的表格输出，并使用 colored 对版本号与更新时间进行了彩色编码（绿色/红色代表版本状态，蓝色/灰色代表活跃度）。
- 运维审计：执行 conn 远程连接时，系统会自动捕获并记录当前 CLI 使用者的身份信息、来源 IP 及目标节点，确保操作可溯源、合规。

## 🚀 快速入门
### 安装
```bash
# 克隆代码库
git clone https://github.com/flystary/xc.git
cd xc

# 构建 Linux 版本 64位
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
# Rust 的编译产物默认在 target/<目标>/release/ 下，我们用 cp 手动重命名移动出来
cp target/x86_64-unknown-linux-gnu/release/xc ./xc

# 构建 macOS 版本 Intel 64位
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/xc ./xc

# 构建 Windows 版本 64位
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/xc.exe ./xc.exe
```

## 使用示例
```bash
# xc

USAGE:
    xc [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    connet    Connect can be used to remotely connect CPE and display the process on the terminal.
    help      Prints this message or the help of the given subcommand(s)
    exec      Use exec to execute commands on the CPE and display the results on the current terminal
    show      Use show to obtain CPE information and display it on the current terminal
    list      Use list to display CPE information


USAGE:
    xc connet [OPTIONS] <sn>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <connetmode>        Use this option to select the remote CPE mode. Otherwise, the default version is SSH.
                           [possible values: ssh, telnet, crt, xshell]
    -m <mode>              Use connet to business the CPE,the default version is nexus. [possible values: nexus, valor,
                           watsons, watsons_ha, tassadar]

ARGS:
    <sn>    cpe serial number

USAGE:
    xc show [OPTIONS] <sn>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m <mode>        Use show to select the CPE, the default version is nexus. [possible values: nexus, valor, watsons,
                     watsons_ha, tassadar]

ARGS:
    <sn>    cpe serial number
```

```

## 技术实现细节
*   **内存级多维索引**：将处理后的数据序列化常驻内存，支持通过 SN、POP 地址等维度构建多级 Map 索引，实现毫秒级搜索响应[cite: 1]。
*   **状态可视化预警**：
    *   **版本显示**：最新版显示为绿色，存在风险的旧版本显示为红色[cite: 1]。
    *   **更新时间**：根据时间戳动态调整颜色（如蓝色/灰色），直观展示节点数据新鲜度[cite: 1]。
*   **多层级跳板穿透**：`conn` 命令支持嵌套多层级 Jump Hosts。无论目标设备隐藏在多少层私有网段之后，CLI 都能自动构建 SSH 隧道进行安全穿透[cite: 1]。

## 🔒 安全合规
所有通过 xc 发起的连接行为均受公司审计系统监控。请确保您的操作符合《SD-WAN 运维合规手册》。
