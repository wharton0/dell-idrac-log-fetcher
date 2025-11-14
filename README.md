# Dell iDRAC Log Fetcher

[![Rust CI](https://github.com/wharton0/dell-idrac-log-fetcher/workflows/Rust%20CI/badge.svg)](https://github.com/wharton0/dell-idrac-log-fetcher/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

自动化连接到 DHCP 模式的 iDRAC 并收集日志的工具。

## ✨ 特性

- 🚀 **自动化 DHCP 服务器** - 无需手动配置网络
- 🔧 **一键日志收集** - 自动运行诊断命令并下载日志
- 💻 **纯 Rust 实现** - 无需 WinPcap/Npcap 驱动
- ⚡ **异步高性能** - 基于 Tokio 异步运行时
- 🎨 **友好界面** - 彩色输出和进度指示器

## 📥 快速开始

### 下载预编译版本（推荐）

**[⬇️ 下载 dell_log_fetcher.exe (v0.1.0)](https://github.com/wharton0/dell-idrac-log-fetcher/releases/download/v0.1.0/dell_log_fetcher.exe)**

或访问 [Releases 页面](https://github.com/wharton0/dell-idrac-log-fetcher/releases) 查看所有版本。

详细的下载和安装说明请参见 [DOWNLOAD.md](DOWNLOAD.md)。

### 从源码编译

```bash
git clone https://github.com/wharton0/dell-idrac-log-fetcher.git
cd dell-idrac-log-fetcher
cargo build --release
```

## 系统要求

### 必需
- Windows 10 或更高版本（x64）
- 管理员权限（用于绑定 DHCP 端口 67）
- Dell RACADM 工具（需要添加到系统 PATH）

### 推荐
- 隔离的网络环境（避免与现有 DHCP 服务器冲突）

## 运行

```bash
# Windows (以管理员身份运行 PowerShell 或 CMD)
.\target\release\dell_log_fetcher.exe
```

**重要**: 必须以管理员权限运行，否则无法绑定 DHCP 端口 67。

右键点击 PowerShell 或 CMD，选择"以管理员身份运行"，然后执行上述命令。

## 使用方法

### 快速开始

1. 编译程序（如果还没有）：
   ```bash
   cargo build --release
   ```

2. 以管理员身份运行：
   - 方法 1: 双击 `run_as_admin.bat`（右键选择"以管理员身份运行"）
   - 方法 2: 以管理员身份打开 PowerShell，运行 `.\target\release\dell_log_fetcher.exe`

3. 按照提示操作：
   - 选择连接到 iDRAC 的网络接口
   - 等待 iDRAC 获取 IP 地址（确保服务器已开机）
   - 输入 iDRAC 凭据（或使用默认值）
   - 等待日志收集完成

### 工作流程

1. **选择网络接口** - 从列表中选择连接到 iDRAC 的网络适配器
2. **启动 DHCP 服务器** - 自动在选定接口上启动 DHCP 服务
3. **等待 iDRAC 请求 IP** - 等待最多 2 分钟让 iDRAC 获取 IP 地址
4. **输入凭据** - 提供 iDRAC 用户名和密码
5. **运行诊断命令** - 自动执行 `getsysinfo`、`getractime` 等命令
6. **收集 TSR 日志** - 触发并下载 SupportAssist 日志包

### 默认凭据

- 用户名: `root`
- 密码: `Password@_`

如果不输入任何内容直接按回车，将使用这些默认值。

## 故障排除

### "Permission denied" 或端口绑定失败
需要以管理员权限运行程序。在 Windows 上右键点击程序选择"以管理员身份运行"。

### "racadm not found"
确保 Dell RACADM 工具已安装并添加到系统 PATH。

### 无法列出网络接口
确保网络适配器已启用并配置了 IPv4 地址。


## 技术细节

### 架构

- **异步 DHCP 服务器**: 使用 Tokio 异步运行时实现高性能 DHCP 服务器
- **无需 WinPcap/Npcap**: 使用纯 Rust 实现，不依赖外部驱动程序
- **跨平台网络接口**: 使用 `if-addrs` 库获取网络接口信息
- **DHCP 协议**: 使用 `dhcproto` 库实现标准 DHCP 协议

### 优化特性

- 代码格式符合 Rust 最佳实践
- 正确的错误处理和资源管理
- 避免跨 await 边界持有 Mutex
- 使用 tokio::select! 实现优雅关闭
- 彩色终端输出提升用户体验
- 进度指示器显示长时间操作状态

## 输出文件

程序会在当前目录生成以下文件：

- `TSR_<IP>_<timestamp>.zip` - SupportAssist 日志包

示例: `TSR_192-168-1-200_20241114_153045.zip`

## 注意事项

1. **管理员权限**: 绑定端口 67 需要管理员权限
2. **防火墙**: 确保防火墙允许 UDP 端口 67 和 68
3. **网络隔离**: 建议在隔离的网络环境中运行，避免与现有 DHCP 服务器冲突
4. **iDRAC 设置**: 确保 iDRAC 网络设置为 DHCP 模式
5. **超时时间**: 默认等待 2 分钟，如果超时请检查物理连接

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📝 许可证

本项目使用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 🔗 相关链接

- [Dell iDRAC 文档](https://www.dell.com/support/kbdoc/en-us/000177080/idrac-home)
- [RACADM 命令行参考](https://www.dell.com/support/manuals/en-us/idrac9-lifecycle-controller-v3.x-series/idrac_3.30.30.30_racadm_pub/racadm-command-line-reference-guide-for-idrac9)

## ⭐ Star History

如果这个项目对你有帮助，请给它一个 Star！
