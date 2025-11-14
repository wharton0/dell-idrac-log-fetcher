# 验证 GitHub Release 的脚本

Write-Host "=== Dell iDRAC Log Fetcher - Release 验证 ===" -ForegroundColor Green
Write-Host ""

# 检查 GitHub CLI
Write-Host "检查 GitHub CLI..." -ForegroundColor Yellow
$ghVersion = gh --version 2>$null
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ GitHub CLI 已安装" -ForegroundColor Green
} else {
    Write-Host "✗ GitHub CLI 未安装" -ForegroundColor Red
    exit 1
}

Write-Host ""

# 获取 Release 信息
Write-Host "获取 Release 信息..." -ForegroundColor Yellow
$release = gh release view v0.1.0 --json name,tagName,url,assets --repo wharton0/dell-idrac-log-fetcher | ConvertFrom-Json

Write-Host "✓ Release 名称: $($release.name)" -ForegroundColor Green
Write-Host "✓ Tag: $($release.tagName)" -ForegroundColor Green
Write-Host "✓ URL: $($release.url)" -ForegroundColor Green

Write-Host ""

# 检查资产
Write-Host "检查 Release 资产..." -ForegroundColor Yellow
$asset = $release.assets | Where-Object { $_.name -eq "dell_log_fetcher.exe" }

if ($asset) {
    Write-Host "✓ 文件名: $($asset.name)" -ForegroundColor Green
    Write-Host "✓ 大小: $([math]::Round($asset.size / 1MB, 2)) MB" -ForegroundColor Green
    Write-Host "✓ SHA256: $($asset.digest)" -ForegroundColor Green
    Write-Host "✓ 下载次数: $($asset.downloadCount)" -ForegroundColor Green
    Write-Host "✓ 下载链接: $($asset.url)" -ForegroundColor Green
} else {
    Write-Host "✗ 未找到 dell_log_fetcher.exe" -ForegroundColor Red
    exit 1
}

Write-Host ""

# 测试下载链接
Write-Host "测试下载链接..." -ForegroundColor Yellow
$downloadUrl = "https://github.com/wharton0/dell-idrac-log-fetcher/releases/download/v0.1.0/dell_log_fetcher.exe"

try {
    $response = Invoke-WebRequest -Uri $downloadUrl -Method Head -UseBasicParsing
    if ($response.StatusCode -eq 200) {
        Write-Host "✓ 下载链接可访问" -ForegroundColor Green
        Write-Host "✓ Content-Type: $($response.Headers['Content-Type'])" -ForegroundColor Green
        Write-Host "✓ Content-Length: $([math]::Round($response.Headers['Content-Length'] / 1MB, 2)) MB" -ForegroundColor Green
    }
} catch {
    Write-Host "✗ 下载链接测试失败: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""

# 检查本地文件
Write-Host "检查本地编译文件..." -ForegroundColor Yellow
$localFile = "target\release\dell_log_fetcher.exe"

if (Test-Path $localFile) {
    $fileInfo = Get-Item $localFile
    $fileHash = Get-FileHash $localFile -Algorithm SHA256
    
    Write-Host "✓ 本地文件存在" -ForegroundColor Green
    Write-Host "✓ 大小: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor Green
    Write-Host "✓ SHA256: $($fileHash.Hash)" -ForegroundColor Green
    Write-Host "✓ 修改时间: $($fileInfo.LastWriteTime)" -ForegroundColor Green
    
    # 比较哈希值
    $remoteHash = $asset.digest -replace "sha256:", ""
    if ($fileHash.Hash -eq $remoteHash.ToUpper()) {
        Write-Host "✓ 本地文件与 Release 文件哈希值匹配" -ForegroundColor Green
    } else {
        Write-Host "⚠ 本地文件与 Release 文件哈希值不匹配" -ForegroundColor Yellow
    }
} else {
    Write-Host "⚠ 本地文件不存在" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "=== 验证完成 ===" -ForegroundColor Green
Write-Host ""
Write-Host "快速下载命令:" -ForegroundColor Cyan
Write-Host "  gh release download v0.1.0 -p 'dell_log_fetcher.exe' -R wharton0/dell-idrac-log-fetcher" -ForegroundColor White
Write-Host ""
Write-Host "或使用 PowerShell:" -ForegroundColor Cyan
Write-Host "  Invoke-WebRequest -Uri '$downloadUrl' -OutFile 'dell_log_fetcher.exe'" -ForegroundColor White
