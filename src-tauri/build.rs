fn main() {
    // 开发模式下跳过图标检查
    #[cfg(debug_assertions)]
    {
        // 设置环境变量跳过资源文件生成
        println!("cargo:rustc-env=TAURI_SKIP_WEBVIEW_INSTALL=1");
    }

    // 使用 try_build 捕获错误
    if let Err(e) = tauri_build::try_build(tauri_build::Attributes::new()) {
        // 在开发模式下，图标缺失只是警告
        #[cfg(debug_assertions)]
        {
            println!("cargo:warning=Tauri build error (ignored in dev): {}", e);
        }

        // 在发布模式下，图标是必需的
        #[cfg(not(debug_assertions))]
        {
            panic!("Tauri build error: {}", e);
        }
    }
}
