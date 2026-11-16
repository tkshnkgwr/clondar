# テストガイドライン (TESTING.md) - Clondar

[English Version](../en/TESTING.md) | **日本語版**

本ドキュメントでは、`Clondar Pro` (v2.0.0 Pure Rust 版) のテスト戦略、品質チェック方針、およびローカル実行手順について解説します。

---

## 1. テスト方針概要

v2.0.0 ではロジック・GUI ともに Rust 単一プロジェクトへ統合されたため、すべてのテストおよび静的解析をルート直下の `cargo` コマンドで一貫して検証します。

主なテスト検証対象は以下の通りです：
- **祝日計算アルゴリズム (`src/holidays.rs`)**:
  - 固定祝日、ハッピーマンデー（新旧制度）、春分の日・秋分の日、天皇誕生日、振替休日・国民の休日・年別特例定義の正確な計算を単体テストで検証します。
- **デフォルト設定整合性・スキーマ検証 (`src/holidays.rs`)**:
  - `config/holidays.json`（`include_str!` 読み込み）が欠損や構文エラーなく読み込めること（`test_default_holidays_json_validity`）、および Serde による不正な月・日・週（範囲外・書式違反）の拒否（`test_validation_error_handling`）を検証します。
- **共有クレート `common_lib` 連携 (`test_holidays_diff_logic`, `test_word_count_logic`)**:
  - 祝日設定の変更差分（LCS Diff）計算と、キーワード集計ロジックの正常動作を検証します。
- **ビルド・インストーラーパッケージ検証**:
  - `winres` (`build.rs`) を含むリリースビルド `cargo build --release` と、NSIS スクリプト `installer/clondar.nsi` のコンパイル成功を確認します。

---

## 2. テストおよび検証実行手順

ローカル開発環境（プロジェクトルート）で以下のコマンドを実行し、コードの安全性を確認します：

### 2.1 単体テスト実行 (Rust)
```bash
cargo test
```

### 2.2 品質・事前検証コマンド群（コミット前チェックリスト）
コード変更時は以下のステップをすべてエラー・警告ゼロで通過することを確認します：

```bash
# 1. フォーマット検査
cargo fmt --check

# 2. 静的解析・Lint（警告をエラーとして扱う）
cargo clippy --all-targets -- -D warnings

# 3. 単体テスト実行
cargo test

# 4. RustDoc 生成確認
cargo doc --no-deps --document-private-items

# 5. 本番リリースビルド（任意・リリース前推奨）
cargo build --release
```

### 2.3 NSIS インストーラー検証（Windows 環境）
```powershell
makensis installer\clondar.nsi
```

---

## 3. テスト追加・保守のルール

1. **法改正に伴う祝日テストの追加**:
   - 日本の祝日法が改正された場合は、`config/holidays.json` の更新に合わせて `src/holidays.rs` 内のユニットテストにも検証ケースを追加してください。
2. **境界値テストの意識**:
   - 年末年始、閏年、振替休日が連続するゴールデンウィーク等の境界ケースを重点的にカバーします。
3. **OS 非依存 / Windows 依存の切り分け**:
   - GitHub Actions 等の CI 環境との互換性や非破壊性を保つため、実環境の状態を汚さない独立したユニットテストとして記述します。
