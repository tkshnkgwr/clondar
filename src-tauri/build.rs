//! # Clondar Pro ビルドスクリプト (build.rs)
//!
//! Tauri v2 のコンパイル時マクロおよび ACL/権限マニフェストの自動生成・リンク処理を行います。

/// Tauri のビルドプロセスを実行します。
///
/// `tauri-build` を呼び出して、フロントエンド資産や設定ファイル (`tauri.conf.json`) のコンテキストを準備します。
fn main() {
    tauri_build::build()
}
