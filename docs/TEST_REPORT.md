# セルフテストレポート & システム構成図 (Obsidian連携)

## 1. セルフテスト結果 (Self-Test Report)

### テスト環境
- **OS**: Windows 11 (低リソース・スペック制限環境を想定したシミュレーション)
- **Runtime**: Tauri v2, Rust 1.96.0, Node.js v26.4.0
- **表示モード**: 透過ウィンドウ、Always on Top (最前面表示固定)、枠なし (Decorations: false)、システムトレイ常駐
- **フロントエンドアーキテクチャ**: Local Bundled SPA (Vite + React)

### テスト項目および検証結果

| テストItemID | テスト項目 | 期待される挙動 | 検証結果 | 判定 |
| :--- | :--- | :--- | :--- | :---: |
| TS-001 | 起動時の位置復元 | 前回の終了位置（絶対物理座標）に正確に復帰すること。 | Tauri v2 規格の `setPosition` への移行、`getCurrentWebviewWindow` によるオブジェクト取得の最適化、および終了時の明示的な位置保存により、DPIスケール環境(125%/150%)を含め、完全に前回と同じ物理位置へ一貫して復帰することを確認。 | **PASS** |
| TS-002 | 起動初期位置競合ガード | 起動直後の自動センター配置とJSの位置復帰処理のラグ中に、一時的な座標を誤検知してLocalStorageを上書き破壊しないこと。 | `isRestoringRef`ガードにより起動後1秒間は監視と書き込みを遮断。無限上書きループを完全終息。 | **PASS** |
| TS-003 | 最前面自動維持 (Always on Top) | ウィンドウを常に最前面に維持し、他のアプリの背後に隠れないこと。 | トグルおよび初期読み込み時に正しく最前面に常時固定されることを検証。 | **PASS** |
| TS-004 | 透過背景 & シャドウ除去 | ウィンドウ背景が完全に透過し、不要な白い枠線やOS標準のドロップシャドウが完全に除去されていること。 | Rustの`set_shadow(false)`呼び出しと設定ファイルの`shadow: false`設定により、完全なボーダーレス・フラット透過が実現。 | **PASS** |
| TS-005 | ドラッグ操作追従 | 背景が透明であっても、data-tauri-drag-regionでのドラッグ移動が滑らかに行えること。 | `pointer-events: auto`の指定により、透過領域でもマウス操作が吸い込まれず正確なドラッグ追従を維持。 | **PASS** |
| TS-006 | セキュリティ特権の検証 | 必要な core API (位置取得・設定、常時最前面) が適切に認可され、ビルドエラーが発生しないこと。 | `allow-set-outer-position` を除外し、Tauri v2規格に合わせた `allow-outer-position` および `allow-set-position` のみに最適化。コンパイル・ビルドが100%成功。 | **PASS** |
| TS-007 | GitHub Actionsリリース検証 | `v*` タグプッシュ時に GitHub Actions の自動ビルド・リリースドラフト作成ワークフローが起動し、文法エラーがないこと。 | YAMLスキーマ検証およびTauri v2のアクション引数との整合性を確認。 | **PASS** |
| TS-008 | Dependabot設定の検証 | CargoとGitHub Actionsの依存関係週次アップデート設定に構文エラーがないこと。 | `.github/dependabot.yml` のスキーマ検証を実施。 | **PASS** |
| TS-009 | エディタ設定の統一 | `.editorconfig` と `.vscode/settings.json` の設定が有効であり、特にPowerShellスクリプトのエンコーディングがBOM付きUTF-8として正しく認識されること。 | VS Codeおよび他エディタでの文字コード・インデント適用の挙動を確認。 | **PASS** |
| TS-010 | システムトレイメニューの動作 | トレイアイコンの右クリックから、ウィンドウ表示切替、最前面切替、位置リセット、終了が機能すること。 | `tray-icon` フィーチャーを有効化し、`main.rs` で `TrayIconBuilder` とメニューイベントハンドラを実装。各項目が正常に動作し、イベントを介してフロントエンドと最前面・位置データが即時同期されることを検証。 | **PASS** |
| TS-011 | 外部祝日の動的ロード | 外部 JSON から祝日の定義を読み込み、正しくカレンダーに描画されること。 | `public/config/holidays.json` をロードし、ハッピーマンデー、天皇誕生日、カスタム上書きを計算・解決してカレンダーに赤字およびツールチップで描画されることを確認。 | **PASS** |
| TS-012 | ローカルビルド（Vite）整合性 | すべてのフロントエンド依存関係がローカルパッケージ化され、オフラインでも正常動作すること。 | `npm install` 実行後に Vite によるビルド（`npm run build`）を行い、オフライン状態でも React/Tailwind/Framer Motion 等の依存関係を含め、アプリが正常に起動・動作することを確認。 | **PASS** |
| TS-013 | 共有クレート (common_lib) 統合 | `common_lib` への依存関係が正しく解決され、単体テストが完全に成功すること。 | `cargo test` の実行により、`it_works`, `test_compute_diff`, `test_count_occurrences` のすべてがパスすることを確認。 | **PASS** |
| TS-014 | 祝日設定マネージャーの動作 | `fallbackConfig` と `holidays.json` の差分を Rust バックエンドで計算し、追加・削除をカラーハイライト表示すること。また統計カウントが正しく表示されること。 | Tauriコマンド `get_holidays_diff` および `get_word_count` が正常に呼び出され、UI 上で追加（緑背景・+）、削除（赤背景・-）、統計情報が正確に表示されることを検証。 | **PASS** |
| TS-015 | 祝日のビジュアル編集（追加・削除） | 祝日設定マネージャー画面で、固定祝日の追加と削除が正常に行え、インメモリで一時保持されること。 | UI フォームに入力して追加、およびゴミ箱アイコンクリックでの削除がインメモリ状態で正確に機能することを確認。 | **PASS** |
| TS-016 | 祝日データのローカル永続化（LocalAppData） | 保存時に `%LOCALAPPDATA%/com.clondar.pro/holidays.json` に正しく上書き保存され、次回起動時やカレンダー表示に即座に反映されること。 | 保存ボタンの押下時に `save_holidays_json` が走り、ファイルが書き換わると同時にカレンダーが即時リロードされ、再起動後も正しく設定が維持されていることを検証。 | **PASS** |

---

## 2. 構成図 (System Architecture Diagram)

以下は、ObsidianなどのMarkdownビューアーで即座に表示可能なMermaid形式のシステム構成図です。

```mermaid
graph TD
    %% スタイリング定義
    classDef frontend fill:#e0f2fe,stroke:#0284c7,stroke-width:2px,color:#0f172a;
    classDef backend fill:#fef3c7,stroke:#d97706,stroke-width:2px,color:#0f172a;
    classDef storage fill:#bbf7d0,stroke:#16a34a,stroke-width:2px,color:#0f172a;
    classDef platform fill:#f1f5f9,stroke:#475569,stroke-width:2px,color:#334155;
    classDef config fill:#edd1ff,stroke:#a855f7,stroke-width:2px,color:#0f172a;

    subgraph OS_Platform [Windows / OS環境]
        TauriRuntime["Tauri v2 Core Webview (Rust runtime)"]:::platform
        TrayAPI["OS System Tray (タスクトレイ)"]:::platform
    end

    subgraph DesktopWidget [低リソースデスクトップウィジェット]
        subgraph FrontEnd [フロントエンド領域 (React / Vite / JSX)]
            WebUI["UI 画面 (ui/index.html)"]:::frontend
            AppJSX["メインロジック (App.jsx)"]:::frontend
            ClockJSX["時計コンポーネント (Clock.jsx)"]:::frontend
            CalendarJSX["カレンダーコンポーネント (Calendar.jsx)"]:::frontend
            HolidaysManager["祝日マネージャー (HolidaysManager.jsx)"]:::frontend
            TauriJS["Tauriラッパー (tauri.js)"]:::frontend
            LocalStorage["localStorage<br/>- windowPosition (Physical座標)<br/>- keepsAlwaysOnTop"]:::storage
        end

        subgraph BackEnd [バックエンド領域 (Rust / Tauri Core / 共有クレート)]
            TauriConf["設定ファイル (tauri.conf.json)<br/>- shadow: false<br/>- transparent: true<br/>- decorations: false"]:::backend
            Capabilities["権限設定 (default.json)<br/>- allow-outer-position<br/>- allow-set-position<br/>- allow-close"]:::backend
            RustMain["Rust ロジック (main.rs)<br/>- System Tray Menu Builder<br/>- rust-version: 1.96.0"]:::backend
            CommonLib["共有クレート (common_lib)<br/>- compute_diff<br/>- count_occurrences"]:::backend
        end
        
        subgraph ConfigLayer [設定データレイヤー]
            HolidaysJSON["外部祝日設定 (LocalAppData/holidays.json)"]:::config
        end
    end

    %% データフローの接続
    WebUI --> AppJSX
    AppJSX --> ClockJSX
    AppJSX --> CalendarJSX
    AppJSX --> HolidaysManager
    CalendarJSX -->|"祝日定義のロード"| HolidaysJSON
    HolidaysManager -->|"Tauri経由での読込・保存"| HolidaysJSON
    HolidaysManager -->|"差分・統計計算コマンド"| TauriRuntime
    TauriRuntime -->|"コマンド実行"| RustMain
    RustMain -->|"共有ロジック呼び出し"| CommonLib
    
    AppJSX -->|"Window位置/最前面設定 復元"| LocalStorage
    TauriJS -->|"位置復元/最前面設定"| TauriRuntime
    
    TauriRuntime -->|"位置/サイズAPIの認可"| Capabilities
    Capabilities -->|"制御シグナル"| TauriConf
    
    RustMain -->|"OSネイティブ描画設定"| TauriRuntime
    RustMain <-->|"トレイメニュー操作"| TrayAPI
    RustMain -->|"イベント送信 (最前面トグル / 位置リセット)"| AppJSX
    
    TauriJS -->|"位置変化検知 (tauri://move)"| LocalStorage
    TauriJS -->|"ドラッグイベント登録"| TauriRuntime
    TauriRuntime -->|"座標変更 / 最前面反映"| OS_Platform

    %% クラスの割当
    class WebUI,AppJSX,ClockJSX,CalendarJSX,TauriJS frontend;
    class TauriConf,Capabilities,RustMain backend;
    class LocalStorage storage;
    class HolidaysJSON config;
```

---
**作成日**: 2026年7月3日
**適合バージョン**: Widget v1.3.0 (Vite React Local Bundled with Rust 1.96.0)
