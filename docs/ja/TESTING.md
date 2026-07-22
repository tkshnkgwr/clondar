# テスト方針・実行ガイド (TESTING.md) - Clondar

[English](../en/TESTING.md) | **日本語版**

本文書は、`Clondar Pro` プロジェクトにおけるユニットテスト、品質検証に関する方針と実行手順をまとめたドキュメントです。

---

## 1. テスト概要

`Clondar Pro` は、バックエンド（Rust / Tauri）およびフロントエンド（React / Vitest）の双方で構成されており、アプリケーションの正確性とウィジェットとしての動作の堅牢性を保証するために包括的なテストを実施します。

主要なテスト観点：
- **祝日日付の算出アルゴリズム (フロントエンド)**:
  - `ui/src/utils/holidays.js` における固定祝日、新旧ハッピーマンデー、春分・秋分の天文計算、振替休日や国民の休日の算出が正確であること（`holidays.test.js` で検証）。
- **データローディングの整合性 (バックエンド)**:
  - 内蔵デフォルト祝日定義JSONの構造検証、ファイル破損時の自動フォールバックが正常に動作すること。
- **共有ライブラリ `common_lib` 連携**:
  - LCSアルゴリズムによる祝日データの行単位テキスト差分（Diff）計算、および統計キーワードカウント処理が正しく動作すること。
- **ウィンドウの物理座標管理**:
  - DPIスケールが異なるマルチモニター環境下において、ウィンドウ終了時の物理座標保存、起動時の復元および初期競合ガード（`isRestoringRef`）が意図通り働くこと。
- **多重起動防止 (Named Mutex)**:
  - 同一OS上でプロセスが複数起動された場合、二重起動を検知して安全に終了すること。

---

## 2. テストの実行手順

ローカル開発環境でのテストおよび品質チェックは、以下の手順で実行します。

### 2.1 バックエンド（Rust）のテスト実行
```bash
# ユニットテストの実行
cargo test --manifest-path src-tauri/Cargo.toml
```

### 2.2 フロントエンド（React / Vitest）のテスト実行
```bash
# フロントエンドのテストスイート実行
npm --prefix ui run test
```

### 2.3 開発完了時のコード検証コマンド
コードのコミットや機能完了時には、以下の標準検証コマンドがすべてエラー・警告なしで合格することを確認してください：

```bash
# 1. バックエンドテスト
cargo test --manifest-path src-tauri/Cargo.toml

# 2. 静的解析 (Clippy)
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings

# 3. コードフォーマット検証
cargo fmt --manifest-path src-tauri/Cargo.toml --check

# 4. Rustdoc ビルド検証
cargo doc --manifest-path src-tauri/Cargo.toml --no-deps --document-private-items
```

---

## 3. テストの記述ガイドライン

1. **祝日ロジックのテスト追加**:
   - 法改正等で新しい祝日区分が追加された場合は、必ず `ui/src/utils/holidays.test.js` にテストケースを追記し、期待値との整合性をテストしてください。
2. **境界値テストの徹底**:
   - 日付計算では年末年始（12月31日〜1月1日）、うるう年、振替休日の多重発生（5月の連休等）など、不具合が発生しやすい境界状態のテストを網羅してください。
3. **OS特有処理の条件付き検証**:
   - Windows 固有の Named Mutex やトレイメニュー関連のテストは、非Windows環境（CI環境など）でエラーにならないよう `#[cfg(windows)]` を適用してください。
