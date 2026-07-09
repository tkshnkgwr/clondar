# Clondar Pro アーキテクチャ設計書 (Tauri v2 Edition)

本ドキュメントは、Clondar Pro のシステムアーキテクチャ、プロセス構造、プロセス間通信（IPC）、データの状態管理、および外部ライブラリとの連携について、技術的な詳細を解説する設計書です。

---

## 1. プロセスモデルと実行ライフサイクル

Tauri v2 フレームワークをベースとしており、セキュリティと安定性の向上のため、**メインプロセス（Rust）**と**レンダラープロセス（Webview / React）**が分離された「マルチプロセスモデル」を採用しています。

```mermaid
graph TD
    subgraph Main_Process [メインプロセス (Rust Backend)]
        RustMain[main.rs]
        TrayCtrl[システムトレイ制御]
        HolidaysCmd[祝日管理コマンド]
        ConfigIO[ファイルI/O]
    end

    subgraph Renderer_Process [レンダラープロセス (React Frontend)]
        ReactApp[App.jsx]
        ClockComp[Clock.jsx]
        CalendarComp[Calendar.jsx]
        HolidaysMgr[HolidaysManager.jsx]
        TauriAPI[Tauri JS API]
    end

    RustMain <-->|"IPC (Commands / Events)"| ReactApp
    TrayCtrl -->|"メニューイベント"| RustMain
    HolidaysCmd -->|"データ処理"| ConfigIO
```

### 1.1 メインプロセス (Rust)
- **役割**: OS ネイティブ機能（ウィンドウの枠・影の制御、システムトレイメニュー、ファイルの永続化、ウィンドウ位置調整）の実行、およびアプリケーションライフサイクル管理。
- **特徴**: セキュアな Rust ランタイム上で動作し、OS 特権が必要なファイルシステムアクセスや API 呼び出しを処理します。

### 1.2 レンダラープロセス (JavaScript/React)
- **役割**: UI の描画、時計アニメーション、カレンダーの組み立て、ユーザー操作のハンドリング。
- **特徴**: OS に内蔵された Webview（Windows の場合は Microsoft Edge WebView2）コンポーネント内で独立して動作し、直接 OS API に触れることはできません。

### 1.3 システムトレイ（常駐化）
- ウィンドウを閉じる操作（❌ボタンや Escキー）が発生した際、ウィンドウを破棄せず「非表示（Hide）」にするフック処理を Rust 側で実装しています。
- アプリケーションの終了は、システムトレイの右クリックメニューの「終了」からのみ実行可能としており、WebView2 終了時の Win32 エラー（Error 1412）を防ぐために、ウィンドウのクローズフローと終了ステート（`QuittingState`）を安全に同期しています。

### 1.4 二重起動防止機能 (Named Mutex)
- OS 上でのアプリケーションの重複起動を防止するため、メインプロセス起動（`main` 関数の最初期フェーズ）において名前付きミューテックス（Named Mutex）を使用した二重起動判定を行っています。
- 共有ライブラリの `common_lib::check_single_instance("com.clondar.pro.mutex", "Clondar")` を呼び出し、既に別インスタンスが起動中で Mutex の取得が失敗（`AlreadyRunning` エラーが発生する）する場合は、エラー出力を伴って即座にプロセスを終了（`std::process::exit(0)`）します。
- これにより、低スペック制限環境などにおいて、メモリ消費の肥大化やイベントバスのシグナル競合を防ぎ、ウィジェットとしての堅牢性を担保しています。

---

## 2. プロセス間通信 (IPC) 設計

メインプロセスとレンダラープロセスは、Tauri が提供する安全なメッセージング境界（IPC）を介して通信します。

### 2.1 Tauri コマンド (Frontend ➔ Backend)
フロントエンドからバックエンドの関数を非同期に呼び出し、結果を受け取るモデルです。
- **`load_holidays_json`**: ローカルデータ領域（LocalAppData）から外部祝日設定ファイルをロードします。
- **`save_holidays_json`**: ユーザーが編集した祝日データをローカルデータ領域へ書き込みます。
- **`get_holidays_diff`**: アプリ内蔵のデフォルト定義と、適用されている外部祝日ファイルの行単位差分を計算します。
- **`get_word_count`**: 祝日定義の出現頻度などの統計情報を計算します。

### 2.2 イベントバス (Backend ➔ Frontend)
バックエンドからフロントエンドへ、非同期に通知イベントをブロードキャストするモデルです。
- **`always-on-top-toggled`**: システムトレイから最前面表示の切替が行われた際、フロントエンドの状態とトレイメニューのチェック表示を同期させます。
- **`position-reset`**: 画面外にウィジェットが紛失した場合などに、トレイメニューの「位置をリセット」からフロントエンドへ座標リセット通知を送信し、中央配置へ復元します。

---

## 3. 状態管理とデータの永続化

Clondar Pro では、データの性質やセキュリティレベルに応じて永続化の手法を使い分けています。

### 3.1 LocalStorage (フロントエンド永続化)
- **対象データ**: 時計の表示モード（12H/24H）、秒表示ON/OFF、時計タイプ（デジタル/アナログ）、最前面表示（ピン留め）状態、および**前回終了時のウィンドウ物理絶対座標**。
- **ウィンドウ位置復旧のロバスト設計**:
  - 高DPI（125%や150%など）やマルチモニター環境での座標ズレを防ぐため、論理座標（Logical）ではなく**物理絶対座標（`Physical`）**を一貫して利用します。
  - **位置検出ガード (`isRestoringRef`)**: 起動時の自動中央配置アニメーションと、JavaScript による位置復元処理が競合し、一時的な中間座標で `LocalStorage` が上書き破壊されるのを防ぐため、起動後 1 秒間は座標の監視・書き込みを遮断します。
  - **終了時位置保存**: アプリケーションクローズ要求の直前に、Tauri API から明示的に `outerPosition` を取得して即時書き込みを行います。

### 3.2 アプリケーションデータ領域 (LocalAppData)
- **対象データ**: 外部祝日設定ファイル (`holidays.json`)。
- **パス**: `%LOCALAPPDATA%/com.clondar.pro/holidays.json`
- **設計方針**: 日本の祝日定義は法改正等によって変化するため、バイナリにハードコードせず外部 JSON に逃がしています。書き込み・読み込みは Rust メインプロセスが管理し、存在しない場合は内蔵のデフォルト定義を自動出力してフォールバックさせます。

---

## 4. 外部ライブラリおよび共有クレート連携

プロジェクトの保守性とコード共有性を最大化するため、複数のプロダクトで共通利用されるアルゴリズムは独立したクレート `common_lib` として切り出されています。

### 4.1 `common_lib` 依存関係の解決
- **Git 依存関係への集約**:  
  GitHub Actions や Dependabot 解析でリポジトリ外の相対パス参照エラー（`path_dependencies_not_reachable`）が発生するのを防ぐため、`src-tauri/Cargo.toml` における `common_lib` の依存定義は Git リポジトリ（`https://github.com/tkshnkgwr/common_lib`）を指定しています。
- **開発時ローカルパス・オーバーライド**:  
  ローカル開発時に `common_lib` と `clondar` を同時修正・並行デバッグできるよう、`src-tauri/.cargo/config.toml` にてオーバーライドを定義し、ローカルの `../../common_lib` を優先してロードする設計となっています。
  ```toml
  paths = ["../../common_lib"]
  ```
  ※ このファイルは開発環境に依存するため、`.gitignore` によって Git 追跡から除外されています。
