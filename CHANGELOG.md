# 更新日志

## v0.1.1 - 2024-11-14

### 改进
- ✅ 移除不必要的 `getconfig -g cfgSerial` 命令
- ✅ 简化诊断命令序列
- ✅ 提升执行效率

### 命令变更
现在只运行以下 racadm 命令：
1. `getsysinfo` - 获取系统信息
2. `getractime` - 获取 iDRAC 时间
3. `supportassist collect` - 触发日志收集
4. `jobqueue view` - 查询任务状态
5. `supportassist exportlastcollection` - 下载日志包

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
