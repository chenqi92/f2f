fn main() {
    // 在开发阶段跳过 Windows 资源文件生成（避免需要图标）
    #[cfg(debug_assertions)]
    {
        std::env::set_var("TAURI_SKIP_WEBVIEW_INSTALL", "true");
        println!("cargo:warning=Development build: skipping icon resource generation");
    }

    tauri_build::build()
}
