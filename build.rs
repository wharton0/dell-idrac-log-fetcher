fn main() {
    // Windows 平台需要链接 Packet.lib (来自 Npcap/WinPcap)
    #[cfg(target_os = "windows")]
    {
        // 尝试常见的 Npcap SDK 安装路径
        let npcap_sdk_paths = vec![
            "C:\\Npcap\\SDK\\Lib\\x64",
            "C:\\Program Files\\Npcap\\SDK\\Lib\\x64",
            "C:\\WpdPack\\Lib\\x64",
        ];

        for path in &npcap_sdk_paths {
            if std::path::Path::new(path).exists() {
                println!("cargo:rustc-link-search=native={}", path);
                break;
            }
        }

        // 如果用户设置了环境变量
        if let Ok(path) = std::env::var("NPCAP_SDK") {
            println!("cargo:rustc-link-search=native={}\\Lib\\x64", path);
        }
    }
}
