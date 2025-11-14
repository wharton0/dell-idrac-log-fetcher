# Release 信息

## ✅ v0.1.0 已成功发布

### 📦 Release 详情

- **版本**: v0.1.0
- **发布日期**: 2024-11-14
- **标签**: v0.1.0
- **状态**: ✅ 已发布

### 📥 下载信息

#### 可执行文件
- **文件名**: `dell_log_fetcher.exe`
- **大小**: 1.54 MB (1,621,504 字节)
- **SHA256**: `7eadbc07860d3a2f438feb48f92470855f0ff6d9333288430823ec950cc7b300`
- **平台**: Windows x64

#### 下载链接

**直接下载**:
```
https://github.com/wharton0/dell-idrac-log-fetcher/releases/download/v0.1.0/dell_log_fetcher.exe
```

**Release 页面**:
```
https://github.com/wharton0/dell-idrac-log-fetcher/releases/tag/v0.1.0
```

### 🔍 验证文件完整性

下载后验证 SHA256 哈希值：

```powershell
Get-FileHash dell_log_fetcher.exe -Algorithm SHA256
```

预期输出：
```
Algorithm       Hash
---------       ----
SHA256          7EADBC07860D3A2F438FEB48F92470855F0FF6D9333288430823EC950CC7B300
```

### 📋 Release 内容

#### 功能特性
- ✅ 自动化 DHCP 服务器用于 iDRAC IP 分配
- ✅ 网络接口选择
- ✅ 自动运行 racadm 诊断命令
- ✅ SupportAssist (TSR) 日志收集
- ✅ 彩色终端输出和进度指示器
- ✅ 无需 WinPcap/Npcap 依赖

#### 技术亮点
- 纯 Rust 实现
- 异步/等待与 Tokio 运行时
- 跨平台网络接口枚举
- 优化的错误处理和资源管理

### 🚀 快速开始

1. **下载文件**
   ```powershell
   # 使用 PowerShell
   Invoke-WebRequest -Uri "https://github.com/wharton0/dell-idrac-log-fetcher/releases/download/v0.1.0/dell_log_fetcher.exe" -OutFile "dell_log_fetcher.exe"
   ```

   或

   ```bash
   # 使用 GitHub CLI
   gh release download v0.1.0 -p "dell_log_fetcher.exe" -R wharton0/dell-idrac-log-fetcher
   ```

2. **验证文件**
   ```powershell
   Get-FileHash dell_log_fetcher.exe -Algorithm SHA256
   ```

3. **运行程序**
   - 右键点击 `dell_log_fetcher.exe`
   - 选择"以管理员身份运行"

### 📝 系统要求

- Windows 10 或更高版本（x64）
- 管理员权限
- Dell RACADM 工具（需要添加到 PATH）

### 📚 文档

- [README.md](README.md) - 完整使用说明
- [DOWNLOAD.md](DOWNLOAD.md) - 详细下载指南
- [CHANGELOG.md](CHANGELOG.md) - 版本历史
- [LICENSE](LICENSE) - MIT 许可证

### 🐛 已知问题

- 需要管理员权限运行
- 仅在 Windows 上测试

### 🔄 更新计划

未来版本可能包含：
- 单元测试和集成测试
- 配置文件支持
- 日志记录功能
- 多 iDRAC 支持
- GUI 界面（可选）

### 📞 支持

如果遇到问题：
1. 查看 [README.md](README.md) 的故障排除部分
2. 搜索 [Issues](https://github.com/wharton0/dell-idrac-log-fetcher/issues)
3. 创建新的 Issue

### 🎉 感谢使用

如果这个工具对你有帮助，请：
- ⭐ 给项目一个 Star
- 🐛 报告 Bug
- 💡 提出新功能建议
- 🤝 贡献代码

---

**项目主页**: https://github.com/wharton0/dell-idrac-log-fetcher
