# 01-tauri: Tauri 2
素の HTML/JS (`src/index.html`) と Rust コマンド (`src-tauri/src/main.rs`)。状態は Rust 側 `Mutex` に保持。

```sh
cd src-tauri && cargo run
```
(`cargo install tauri-cli` 済みなら `cargo tauri dev` も可)
