# 更新日志

## v0.1.0 - 2024-11-14

### 初始版本

#### 功能
- ✅ 自动 DHCP 服务器用于 iDRAC IP 分配
- ✅ 网络接口选择
- ✅ 自动运行 racadm 诊断命令
- ✅ SupportAssist (TSR) 日志收集
- ✅ 彩色终端输出
- ✅ 进度指示器

#### 技术改进
- ✅ 移除 pnet 依赖，避免 WinPcap/Npcap 要求
- ✅ 使用 if-addrs 进行跨平台网络接口枚举
- ✅ 更新到 dhcproto 0.12 API
- ✅ 修复 MutexGuard 跨 await 边界问题
- ✅ 实现优雅的服务器关闭机制
- ✅ 代码格式化和优化

#### 已知问题
- 需要管理员权限运行
- 仅在 Windows 上测试

#### 依赖项
- tokio 1.x (异步运行时)
- dhcproto 0.12 (DHCP 协议)
- if-addrs 0.13 (网络接口)
- colored 2.1 (彩色输出)
- indicatif 0.17 (进度条)
- rpassword 7.3 (密码输入)
- chrono 0.4 (时间戳)
