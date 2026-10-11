# rust-gui-comparison

同じ仕様の TODO アプリ（追加 / 完了トグル / 削除 / 残件数表示、メモリ内のみ）を、Rust の主要な GUI 手法で実装して比較するサンプル集です。

比較結果は [COMPARISON.md](COMPARISON.md)（Markdown）と [COMPARISON.html](COMPARISON.html)（ブラウザで開く単一ファイル）にまとめています。

## 構成

| フォルダ | 内容 | 使用バージョン | 起動 |
|---|---|---|---|
| [`00-basic`](00-basic) | Rust 言語の主要機能（CLI） | rustc 1.99.0 | `cargo run` / `cargo test` |
| [`01-tauri`](01-tauri) | Tauri（WebView + HTML/JS + Rust） | tauri 2.12.2 | `cd src-tauri && cargo run` |
| [`01-tauri-react`](01-tauri-react) | Tauri + React（Vite + TypeScript） | tauri 2.12.2 / react 19 | `npm install && npm run tauri dev` |
| [`02-egui`](02-egui) | egui / eframe（即時モード） | eframe 0.36.2 | `cargo run` |
| [`03-iced`](03-iced) | Iced（Elm アーキテクチャ） | iced 0.14.0 | `cargo run` |
| [`04-slint`](04-slint) | Slint（宣言的 DSL） | slint 1.18.1 | `cargo run` |
| [`05-dioxus`](05-dioxus) | Dioxus desktop（RSX + signals） | dioxus 0.7.10 | `cargo run` |
| [`06-leptos`](06-leptos) | Leptos CSR（WASM、ブラウザ） | leptos 0.8.22 | `trunk serve --open` |
| [`07-gtk4`](07-gtk4) | GTK4（gtk4-rs） | gtk4 0.11.5 | `cargo run` |

各フォルダは独立した Cargo プロジェクトです（ワークスペースではありません）。そのため、実行は各フォルダに移動して行います。

```sh
cd 02-egui
cargo run
```

## 前提条件

- Rust（検証時は 1.99.0）。`rustup` で導入してください。
- 共通で追加の準備が要らないもの: `00-basic`, `02-egui`, `03-iced`, `04-slint`, `05-dioxus`
- `01-tauri-react`: Node.js と npm が必要です（検証時は Node 22）。
- `01-tauri`: OS の WebView が必要です（macOS / Windows は標準。Linux は webkit2gtk）。
- `06-leptos`: 次を事前に実行してください。
  ```sh
  rustup target add wasm32-unknown-unknown
  cargo install trunk --locked
  ```
- `07-gtk4`: GTK4 本体が必要です。macOS では `brew install gtk4`（`pkg-config` も必要）。

## 検証状況

- 全アプリで `cargo build` とリリースビルドが成功しています（Leptos は `trunk build --release`）。
- `00-basic` は `cargo test` が 3 件成功しています。
- **ウィンドウを起動しての動作確認は行っていません。** 見た目や操作（IME 入力、キー操作など）は未確認です。
- 検証環境: macOS（Apple Silicon）、2026-10 時点の各安定版。

## 補足

- 初回ビルドは依存クレートの取得とコンパイルのため時間がかかります。`target/` は各アプリで 1〜4 GB 程度になります（`.gitignore` 済み）。
- 0.x のフレームワークは破壊的変更が頻繁です。バージョンを変えるとコードの修正が必要になることがあります（例: eframe 0.36 では `App::update` が廃止され `App::ui` になりました）。
- 比較は小さな TODO アプリ 1 つに基づく印象であり、大規模アプリでの優劣を示すものではありません。

## ライセンス

このリポジトリのソースコードは [MIT License](LICENSE) です。依存するフレームワーク（Tauri、egui、Iced、Slint、Dioxus、Leptos、gtk4-rs など）は、それぞれ独自のライセンスで提供されています。特に Slint は選択式（GPL-3.0 / ロイヤリティフリー / 商用）のため、ビルドしたバイナリを配布する場合は各ライセンスに従ってください。
