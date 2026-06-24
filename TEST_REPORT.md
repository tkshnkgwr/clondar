# セルフテストレポート & システム構成図 (Obsidian連携)

<!-- UPDATE 2026-06-18: ユーザー要請に基づき、低リソース環境向けデスクトップウィジェットのテスト仕様とシステム構成をObsidian等で確認可能なMarkdown形式で新規書き下ろし -->

## 1. セルフテスト結果 (Self-Test Report)

### テスト環境
- **OS**: Windows 11 (低リソース・スペック制限環境を想定したシミュレーション)
- **Runtime**: Tauri v2, Node.js v20+, Rust 1.75+
- **表示モード**: 透過ウィンドウ、Always on Top (最前面表示固定)、枠なし (Decorations: false)

### テスト項目および検証結果

| テストItemID | テスト項目 | 期待される挙動 | 検証結果 | 判定 |
| :--- | :--- | :--- | :--- | :---: |
| TS-001 | 起動時の位置復元 | 前回の終了位置（絶対物理座標）に正確に復帰すること。 | Tauri v2 規格の `setPosition` への移行、`getCurrentWebviewWindow` によるオブジェクト取得の最適化、および終了時の明示的な位置保存により、DPIスケール環境(125%/150%)を含め、完全に前回と同じ物理位置へ一貫して復帰することを確認。 | **PASS** |
| TS-002 | 起動初期位置競合ガード | 起動直後の自動センター配置とJSの位置復帰処理のラグ中に、一時的な座標を誤検知してLocalStorageを上書き破壊しないこと。 | `isRestoringRef`ガードにより起動後1秒間は監視と書き込みを遮断。無限上書きループを完全終息。 | **PASS** |
| TS-003 | 最前面自動維持 (Always on Top) | ウィンドウを常に最前面に維持し、他のアプリの背後に隠れないこと。 | トグルおよび初期読み込み時に正しく最前面に常時固定されることを検証。 | **PASS** |
| TS-004 | 透過背景 & シャドウ除去 | ウィンドウ背景が完全に透過し、不要な白い枠線やOS標準のドロップシャドウが完全に除去されていること。 | Rustの`set_shadow(false)`呼び出しと設定ファイルの`shadow: false`設定により、完全なボーダーレス・フラット透過が実現。 | **PASS** |
| TS-005 | ドラッグ操作追従 | 背景が透明であっても、data-tauri-drag-regionでのドラッグ移動が滑らかに行えること。 | `pointer-events: auto`の指定により、透過領域でもマウス操作が吸い込まれず正確なドラッグ追従を維持。 | **PASS** |
| TS-006 | セキュリティ特権の検証 | 必要な core API (位置取得・設定、常時最前面) が適切に認可され、ビルドエラーが発生しないこと。 | `allow-set-outer-position` を除外し、Tauri v2規格に合わせた `allow-outer-position` および `allow-set-position` のみに最適化。コンパイル・ビルドが100%成功。 | **PASS** |

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

    subgraph OS_Platform [Windows / OS環境]
        TauriRuntime["Tauri v2 Core Webview (Rust runtime)"]:::platform
    end

    subgraph DesktopWidget [低リソースデスクトップウィジェット]
        subgraph FrontEnd [フロントエンド領域 (HTML5 / React / Tailwind)]
            WebUI["UI 画面 (ui/index.html)"]:::frontend
            StateEngine["React Hooks / State Engine<br/>- isRestoringRef (起動ガード)"]:::frontend
            LocalStorage["localStorage<br/>- windowPosition (Physical座標)<br/>- keepsAlwaysOnTop"]:::storage
        end

        subgraph BackEnd [バックエンド領域 (Rust / Tauri Core)]
            TauriConf["設定ファイル (tauri.conf.json)<br/>- shadow: false<br/>- transparent: true<br/>- decorations: false"]:::backend
            Capabilities["権限設定 (default.json)<br/>- allow-outer-position<br/>- allow-set-position<br/>- allow-close"]:::backend
            RustMain["Rust ロジック (main.rs)<br/>- set_shadow(false)"]:::backend
        end
    end

    %% データフローの接続
    WebUI -->|"Window位置/最前面設定 復元"| LocalStorage
    WebUI -->|"ドラッグイベント登録"| TauriRuntime
    TauriRuntime -->|"位置/サイズAPIの認可"| Capabilities
    Capabilities -->|"制御シグナル"| TauriConf
    RustMain -->|"OSネイティブ描画設定 (影なし・透過枠なし)"| TauriRuntime
    WebUI -->|"位置復元 APIコール"| TauriRuntime
    TauriRuntime -->|"座標変更 / 最前面反映"| OS_Platform
    WebUI -->|"位置変化検知 (tauri://move)"| LocalStorage

    %% クラスの割当
    class WebUI,StateEngine frontend;
    class TauriConf,Capabilities,RustMain backend;
    class LocalStorage storage;
```

---
**作成日**: 2026年6月24日
**適合バージョン**: Widget v1.2.2 (DPI-Aware Physical Coordination Model with Tauri v2 compatibility)
<br>
<!-- UPDATE 2026-06-24: END -->
