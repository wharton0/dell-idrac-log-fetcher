# 下载和安装指南

## 📥 快速下载

### 方式 1: 从 GitHub Release 下载（推荐）

1. 访问 [Releases 页面](https://github.com/wharton0/dell-idrac-log-fetcher/releases)
2. 下载最新版本的 `dell_log_fetcher.exe`
3. 保存到你想要的目录

**直接下载链接**: [dell_log_fetcher.exe (v0.1.0)](https://github.com/wharton0/dell-idrac-log-fetcher/releases/download/v0.1.0/dell_log_fetcher.exe)

### 方式 2: 使用 GitHub CLI

```bash
gh release download v0.1.0 -p "dell_log_fetcher.exe" -R wharton0/dell-idrac-log-fetcher
```

### 方式 3: 使用 PowerShell 下载

```powershell
# 下载到当前目录
Invoke-WebRequest -Uri "https://github.com/wharton0/dell-idrac-log-fetcher/releases/download/v0.1.0/dell_log_fetcher.exe" -OutFile "dell_log_fetcher.exe"
```

### 方式 4: 使用 curl

```bash
curl -L -o dell_log_fetcher.exe https://github.com/wharton0/dell-idrac-log-fetcher/releases/download/v0.1.0/dell_log_fetcher.exe
```

## 📋 文件信息

- **文件名**: dell_log_fetcher.exe
- **大小**: 1.54 MB (1,621,504 字节)
- **SHA256**: `7eadbc07860d3a2f438feb48f92470855f0ff6d9333288430823ec950cc7b300`
- **版本**: v0.1.0
- **平台**: Windows x64

## ✅ 验证文件完整性

下载后，你可以验证文件的 SHA256 哈希值：

```powershell
# PowerShell
Get-FileHash dell_log_fetcher.exe -Algorithm SHA256
```

预期输出：
```
Algorithm       Hash
---------       ----
SHA256          7EADBC07860D3A2F438FEB48F92470855F0FF6D9333288430823EC950CC7B300
```

## 🚀 安装和运行

### 1. 下载文件
使用上述任一方式下载 `dell_log_fetcher.exe`

### 2. 放置文件
将文件放到一个方便的位置，例如：
- `C:\Tools\dell_log_fetcher.exe`
- `C:\Program Files\DellTools\dell_log_fetcher.exe`
- 或任何你喜欢的目录

### 3. 运行程序

**重要**: 必须以管理员权限运行！

#### 方法 A: 右键菜单
1. 右键点击 `dell_log_fetcher.exe`
2. 选择"以管理员身份运行"

#### 方法 B: PowerShell（管理员）
```powershell
# 以管理员身份打开 PowerShell
cd C:\path\to\your\directory
.\dell_log_fetcher.exe
```

#### 方法 C: CMD（管理员）
```cmd
REM 以管理员身份打开 CMD
cd C:\path\to\your\directory
dell_log_fetcher.exe
```

## 🔧 系统要求

### 必需
- Windows 10 或更高版本（x64）
- 管理员权限
- Dell RACADM 工具（需要添加到 PATH）

### 可选
- 网络连接到 iDRAC
- 隔离的网络环境（推荐）

## 📝 首次使用

1. 以管理员身份运行程序
2. 从列表中选择连接到 iDRAC 的网络接口
3. 确保 Dell 服务器已开机且 iDRAC 设置为 DHCP 模式
4. 等待程序自动分配 IP 地址
5. 输入 iDRAC 凭据（默认: root / Password@_）
6. 等待日志收集完成

## ❓ 常见问题

### Q: 为什么需要管理员权限？
A: 程序需要绑定 UDP 端口 67（DHCP 服务器端口），这需要管理员权限。

### Q: 杀毒软件报警怎么办？
A: 这是误报。程序是开源的，你可以查看源代码。如果不放心，可以自己编译。

### Q: 如何自己编译？
A: 参见 [README.md](README.md) 的编译部分。

### Q: 支持其他操作系统吗？
A: 当前版本仅支持 Windows。理论上可以支持 Linux/macOS，但需要额外开发。

## 🔄 更新

要更新到新版本：
1. 下载新版本的 exe 文件
2. 替换旧文件
3. 无需卸载或其他操作

## 🗑️ 卸载

直接删除 `dell_log_fetcher.exe` 文件即可。程序不会在系统中留下任何其他文件（除了生成的日志文件）。

## 🆘 获取帮助

如果遇到问题：
1. 查看 [README.md](README.md) 的故障排除部分
2. 在 [Issues](https://github.com/wharton0/dell-idrac-log-fetcher/issues) 页面搜索类似问题
3. 创建新的 Issue 描述你的问题

## 📞 联系方式

- GitHub Issues: https://github.com/wharton0/dell-idrac-log-fetcher/issues
- GitHub Discussions: https://github.com/wharton0/dell-idrac-log-fetcher/discussions

---

**享受使用！如果这个工具对你有帮助，请给项目一个 ⭐ Star！**
