//! # Clondar Pro ビルドスクリプト (build.rs)
//!
//! Windows リリースバイナリ (`Clondar.exe`) へアプリケーションアイコン (`icons/icon.ico`)
//! および製品名・バージョン・著作権などのリソース情報を埋め込みます。

fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=icons/icon.ico");
        let mut res = winres::WindowsResource::new();
        res.set_icon("icons/icon.ico");
        res.set("ProductName", "Clondar Pro");
        res.set(
            "FileDescription",
            "Clondar Pro - Minimalist Desktop Clock & Calendar Widget",
        );
        res.set("LegalCopyright", "Copyright (c) 2026 tkshnkgwr");
        res.compile()
            .expect("Windows リソース (icons/icon.ico) の埋め込みに失敗しました");
    }
}
