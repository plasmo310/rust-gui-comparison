# 01-tauri-react: Tauri 2 + React (Vite + TypeScript)

公式テンプレートに近い構成です。

フロントは `src/`（React）、Rust は `src-tauri/`（`lib.rs` + `main.rs`）。

Rust 側のコマンド（add / toggle / remove / list）は `01-tauri` と同じです。

```sh
npm install
npm run tauri dev      # 開発（Vite 開発サーバ + Tauri）
npm run build          # フロントのビルド（dist/ を生成）
cd src-tauri && cargo build   # 先に npm run build が必要（dist/ を埋め込むため）
```
