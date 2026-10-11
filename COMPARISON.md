# Rust GUI フレームワーク比較: 同一 TODO アプリを 7 通りで実装

同じ仕様（追加 / 完了トグル / 削除 / 残件数表示、メモリ内のみ）の TODO アプリを、主要な Rust GUI 手法で実装して比較しました。`00-basic` は GUI なしで Rust 言語自体の主要機能を見る導入サンプルです。

| フォルダ | 手法 | 使用バージョン |
|---|---|---|
| `00-basic` | Rust 言語の主要機能 (CLI) | rustc 1.99.0 |
| `01-tauri` | Tauri (WebView + HTML/JS + Rust) | tauri 2.12.2 |
| `02-egui` | egui / eframe (即時モード) | eframe 0.36.2 |
| `03-iced` | Iced (Elm アーキテクチャ) | iced 0.14.0 |
| `04-slint` | Slint (宣言的 DSL) | slint 1.18.1 |
| `05-dioxus` | Dioxus desktop (RSX + signals) | dioxus 0.7.10 |
| `06-leptos` | Leptos CSR (WASM、ブラウザ) | leptos 0.8.22 + trunk |
| `07-gtk4` | GTK4 (gtk4-rs、ネイティブ) | gtk4 0.11.5 |

## 1. 検証状況

| 手法 | `cargo build` | リリースビルド | GUI 起動確認 |
|---|---|---|---|
| 00-basic | OK | OK (`cargo test` 3 件成功、`cargo run` で出力確認) | 該当なし |
| 01-tauri | OK | OK | 未実施 |
| 02-egui | OK | OK | 未実施 |
| 03-iced | OK | OK | 未実施 |
| 04-slint | OK | OK | 未実施 |
| 05-dioxus | OK | OK | 未実施 |
| 06-leptos | OK (wasm32) | OK (`trunk build --release`) | 未実施 |
| 07-gtk4 | OK | OK | 未実施 |

> **注意**: 検証はビルド成功までです。ウィンドウを実際に起動しての操作確認はしていません。画面上の見た目や挙動（IME 入力、キー操作など）は未確認です。

## 2. 実測値

測定環境: macOS (Apple Silicon)、Rust 1.99.0、各アプリのリリースビルドを `target` なしの状態に近い条件（依存クレートのソースはキャッシュ済み）で計測。

| 手法 | 実装行数 (Rust + UI) | 依存パッケージ数 (Cargo.lock) | リリースビルド時間 | リリース成果物 | `target/` 合計 (dev+release) |
|---|---|---|---|---|---|
| 00-basic | 223 | 1 | <1 秒 | 518 KB | 12 MB |
| 01-tauri | 59 + HTML/JS 46 = 105 | 407 | 70 秒 | 9.6 MB | 2,160 MB |
| 02-egui | 46 | 418 | 41 秒 | 13.1 MB | 2,212 MB |
| 03-iced | 70 | 383 | 34 秒 | 10.6 MB | 1,903 MB |
| 04-slint | 54 + .slint 34 = 88 | 568 | 60 秒 | 15.6 MB | 3,827 MB |
| 05-dioxus | 61 | 543 | 46 秒 | 6.0 MB | 2,440 MB |
| 06-leptos | 52 + HTML 9 = 61 | 181 | 計測不可 (注1) | wasm 341 KB (+ JS 30 KB) | 1,767 MB |
| 07-gtk4 | 102 | 65 | 42 秒 | 515 KB (注2) | 965 MB |

- 注1: 初回の `trunk build --release` はツールの取得などを含み約 44 分かかりましたが、待ち時間の内訳を切り分けていないためコンパイル時間としては採用していません。
- 注2: GTK4 は gtk4 ライブラリ (Homebrew) を動的リンクするため、配布時は別途ランタイムが必要です。バイナリが小さいのはそのためです。
- 行数は `wc -l` による単純な行数です（空行・コメント含む）。ビルド時間は 1 回の計測値で、マシン状態により変動します。
- 依存数・ビルド時間はコードの書き方よりも依存クレートの規模で決まります。

## 3. 開発環境構築コスト

Rust（`rustup`）と C リンカ（macOS: Xcode Command Line Tools / Windows: MSVC Build Tools / Linux: `build-essential`）は全手法で共通です。以下は**それ以外に必要なもの**です。

- macOS: 今回の環境で実際に実行して確認しました。
- Linux / Windows: 公式ドキュメント（2026-10-10 取得）の記載です。実機では検証していません。

### 3-1. 追加で必要なもの（OS 別）

| 手法 | macOS | Linux (Debian/Ubuntu) | Windows | コスト目安 (mac / Linux / Win) |
|---|---|---|---|---|
| Tauri | 追加なし | `apt install` 8 パッケージ（libwebkit2gtk-4.1-dev, libxdo-dev, libssl-dev, libayatana-appindicator3-dev, librsvg2-dev ほか） | WebView2（Win10 1803 以降は標準） | 低 / 中 / 低〜中 |
| egui | 追加なし | 公式 README: `apt install` 6 パッケージ（libclang-dev, libgtk-3-dev, libxcb-*-dev, libxkbcommon-dev, libssl-dev）※デモアプリ向けの記載 | 追加なし（公式記載） | 低 / 中 / 低 |
| Iced | 追加なし | 公式 README に記載なし（未確認） | 公式 README に記載なし | 低 / 不明 / 低 |
| Slint | 追加なし | 公式 README に記載なし（未確認） | 公式 README に記載なし | 低 / 不明 / 低 |
| Dioxus | 追加なし | `apt install` 9 パッケージ（Tauri と同じ + lld） | WebView2（Edge があれば可） | 低 / 中 / 低〜中 |
| Leptos | `rustup target add wasm32-unknown-unknown` と `cargo install trunk --locked` | 同左 | 同左 | 中 / 中 / 中 |
| GTK4 | `brew install gtk4` | `apt install libgtk-4-dev libadwaita-1-dev meson ...`（公式 book） | gvsbuild で GTK をビルドし環境変数を 5 項目設定（最短の公式手順）。他に MSYS2 / 手動ビルドの選択肢あり | 低 / 中 / 高 |

コスト目安は、追加の手順数と難しさに基づく筆者の判断です。「低」は追加なし、「中」はパッケージ導入 1〜2 コマンド、「高」は多段の手順や環境変数設定が必要、を指します。

### 3-2. ゼロから動かすまでの手順（macOS）

共通（1 回だけ）:
```sh
xcode-select --install
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

| 手法 | 追加の準備 | 実行 |
|---|---|---|
| Tauri | なし | `cd 01-tauri/src-tauri && cargo run` |
| egui | なし | `cd 02-egui && cargo run` |
| Iced | なし | `cd 03-iced && cargo run` |
| Slint | なし | `cd 04-slint && cargo run` |
| Dioxus | なし | `cd 05-dioxus && cargo run` |
| Leptos | `rustup target add wasm32-unknown-unknown`<br>`cargo install trunk --locked` | `cd 06-leptos && trunk serve --open` |
| GTK4 | `brew install gtk4` | `cd 07-gtk4 && cargo run` |

### 3-3. 任意の CLI

必須ではありませんが、公式が案内している補助ツールです。

| ツール | 用途 | 備考 |
|---|---|---|
| `cargo install tauri-cli`（`cargo tauri`） | Tauri の開発サーバ・バンドル | 今回の構成（素の HTML）では不要 |
| `dx`（dioxus-cli） | ホットリロード・バンドル | 公式によるとソースからのビルドは最大 10 分かかるため、ビルド済みバイナリを推奨 |
| Node.js / npm | Tauri で JS フレームワークを使う場合 | 今回の構成では不要 |

### 3-4. 初回ビルド時間（参考）

デバッグビルド（依存クレートの取得を含む）を 1 回測った値です。キャッシュの状態で変わるため、目安としてだけ見てください。

| 手法 | 初回 `cargo build` |
|---|---|
| Tauri | 80 秒 |
| egui | 約 20 秒 * |
| Iced | 24 秒 |
| Slint | 29 秒 |
| Dioxus | 17 秒 |
| Leptos | 46 秒 (wasm32) |
| GTK4 | 23 秒 |

\* egui は最初のビルドがソースの API 変更でコンパイルエラーになったため、依存クレートのビルドまでの時間です。

### 3-5. 実際に詰まった点

- **Tauri:** `icons/icon.png` が無いとビルドできません。設定ファイル（`tauri.conf.json`）と権限定義（`capabilities`）も必要で、準備物が他より多いです。
- **egui / eframe 0.36:** `App::update(ctx, frame)` が廃止され `App::ui(ui, frame)` になりました。`CentralPanel::show_inside` も `show` に改名されています。古い記事のコードはそのままではコンパイルできません。
- **Leptos:** ブラウザ向け WASM のため `cargo run` では動かず、`trunk` と `wasm32` ターゲットが前提です。
- **GTK4:** macOS は `brew install gtk4` の 1 コマンドで済みましたが、Windows の公式手順は最短でも GTK のビルドと環境変数設定が必要で、最も重いです。

### 3-6. 総合評価

環境構築が軽い順（macOS / Linux / Windows をまとめた印象）: **egui ≒ Iced ≒ Slint**（ほぼ `cargo run` だけ）→ **Dioxus ≒ Tauri**（Linux でパッケージ導入）→ **Leptos**（`trunk` と WASM ターゲット）→ **GTK4**（特に Windows）。

## 4. アーキテクチャと実装方法の違い

| 手法 | 描画方式 | 状態管理 | UI の書き方 | 言語 |
|---|---|---|---|---|
| Tauri | OS の WebView (DOM) | Rust 側 `Mutex<Store>` + コマンドで JS から操作 | HTML/CSS/JS | Rust + JS |
| egui | 即時モード (毎フレーム再構築、GPU) | 普通の構造体のフィールド | `ui.button()` などを毎フレーム呼ぶ | Rust |
| Iced | 保持モード + wgpu | `State` + `Message` + `update` (Elm) | `view` が `Element` を返す | Rust |
| Slint | 独自レンダラ (GPU / ソフトウェア) | `.slint` のプロパティ + Rust の `VecModel` | 宣言的 DSL + コールバック | Slint DSL + Rust |
| Dioxus | WebView (DOM、デスクトップ版) | `use_signal` (リアクティブ) | `rsx!` マクロ (React 風) | Rust |
| Leptos | ブラウザ DOM (WASM、細粒度リアクティビティ) | `signal` / `RwSignal` | `view!` マクロ (JSX 風) | Rust |
| GTK4 | ネイティブ GTK (命令的) | `Rc<RefCell<State>>` を手動共有 | ウィジェットを生成して `append` | Rust |

### 同じ「追加」処理の比較

**egui** (即時モード: 状態を直接書き換える)
```rust
if (ui.button("追加").clicked() || enter) && !self.input.trim().is_empty() {
    self.todos.push((self.input.trim().to_string(), false));
    self.input.clear();
}
```

**Iced** (メッセージを送り `update` で処理)
```rust
Message::Add => {
    let t = self.input.trim();
    if !t.is_empty() {
        self.todos.push(Todo { title: t.to_string(), done: false });
        self.input.clear();
    }
}
```

**Slint** (UI は DSL、ロジックはコールバック登録)
```rust
ui.on_add(move |title| {
    todos.push(TodoItem { title: title.trim().into(), done: false });
    update();
});
```

**Dioxus / Leptos** (シグナルを更新すると UI が自動更新)
```rust
// Dioxus
todos.write().push(Todo { id, title: t, done: false });
// Leptos
set_todos.update(|v| v.push(Todo { id, title: t, done: RwSignal::new(false) }));
```

**Tauri** (JS から Rust のコマンドを呼ぶ)
```rust
#[tauri::command]
fn add(state: State<AppState>, title: String) -> Vec<Todo> { /* push して全件を返す */ }
```
```js
render(await invoke("add", { title: $("input").value }));
```

**GTK4** (状態変更後に UI を手動で再構築)
```rust
state.borrow_mut().todos.push(Todo { title: t, done: false });
render(&state); // 子ウィジェットを全削除して作り直す
```

## 5. 長所・短所

| 手法 | 長所 | 短所 |
|---|---|---|
| Tauri | Web 技術資産を再利用できる。Rust 側に処理を隔離。成果物は比較的小さい | フロントと Rust の二言語。設定ファイルが多い。WebView の差異（OS ごと）に依存 |
| egui | 最少行数・最小セットアップ。ツール/デバッグ用 UI に最適 | ネイティブらしい見た目ではない。アクセシビリティ・IME・複雑なレイアウトは弱め |
| Iced | 状態遷移が明快でテストしやすい。Rust のみで完結 | API の変更が多い。ウィジェットが比較的少ない |
| Slint | UI と処理の分離が明確。組込みからデスクトップまで対応 | DSL の習得が必要。ライセンス（GPL / ロイヤリティフリー / 商用）の確認が必要。ビルドが重め |
| Dioxus | React 経験者に馴染む。Web/モバイルと共通化しやすい | バージョンが若く API が動く。デスクトップは WebView 依存 |
| Leptos | 依存が少なく WASM が非常に小さい。細粒度リアクティブで効率的 | ネイティブアプリではない（ブラウザ向け）。`trunk` と WASM ターゲットが必要 |
| GTK4 | 本物のネイティブ UI（Linux の標準）。依存パッケージが少ない | macOS/Windows では導入が面倒。命令的で記述量が多く、状態同期は手動 |

## 6. 普及度・成熟度（選定用データ）

2026-10-10 に crates.io API と GitHub API から取得した実測値です（スター数・ダウンロード数は取得時点のスナップショット）。

| 手法 | GitHub ★ | フォーク | 未解決 Issue | crates.io 累計DL | 直近90日DL | 最新安定版 | 最終リリース | 初版 |
|---|---|---|---|---|---|---|---|---|
| Tauri | 111,721 | 4,053 | 1,491 | 35.4M | 13.9M | 2.12.2 | 2026-10-09 | 2019 |
| egui (eframe) | 31,140 | 2,184 | 1,037 | 20.1M (egui 本体 25.8M) | 5.0M | 0.36.2 | 2026-09-08 | 2020 |
| Iced | 31,711 | 1,699 | 510 | 2.9M | 0.64M | 0.14.0 | 2025-12-07 | 2019 |
| Slint | 24,126 | 1,028 | 827 | 1.9M | 0.61M | 1.18.1 | 2026-09-21 | 2022 |
| Dioxus | 39,356 | 1,919 | 821 | 3.0M | 1.15M | 0.7.10 (0.8.0-alpha あり) | 2026-07-31 | 2021 |
| Leptos | 21,374 | 904 | 112 | 5.1M | 1.40M | 0.8.22 | 2026-10-05 | 2022 |
| GTK4 (gtk4-rs) | 2,355 | 217 | 78 | 4.8M | 1.57M | 0.11.5 | 2026-09-20 | 2019 |

読み取り方:

- **ダウンロード数は「シェア」ではありません。** crates.io のDLにはCI・他クレートからの間接依存も含まれます（例: gtk4 は他の GTK 系クレートから、egui/eframe はエコシステムのクレートから引かれます）。公式の「利用率」統計は存在しないため、人気の目安としてのみ使ってください。
- スター数は Tauri が突出（約11万）。Dioxus、Iced、egui が 3〜4 万台で続き、Slint、Leptos が約 2 万台。gtk4-rs は GTK 本体の周辺ラッパーのため数字が小さいですが、GTK 自体は長年の実績があります。
- **1.0 未満のものが多い**: egui / Iced / Dioxus / Leptos / gtk4-rs は 0.x で、マイナーバージョンごとに破壊的変更が入ります（実際に egui 0.36 で `App::update` が廃止されました）。1.0 以上なのは Tauri 2.x と Slint 1.x のみです。
- Iced は GitHub のコミットは活発ですが、crates.io の最終リリースは約 10 か月前です。

### 機能・ライセンス・対応範囲

以下は各プロジェクトの公開情報に基づく一般的な整理で、今回のコードでは検証していません。導入前に公式ドキュメントで最新状況を確認してください。

| 手法 | ライセンス | 対応ターゲット | 向いている規模 | 備考 |
|---|---|---|---|---|
| Tauri | Apache-2.0 / MIT | Windows, macOS, Linux, iOS, Android | 小〜大 | WebView 利用。フロントは任意の Web フレームワーク |
| egui | Apache-2.0 / MIT | デスクトップ, Web (WASM), Android | 小〜中 | ツール/ゲーム内 UI に多い。アクセシビリティは AccessKit 経由 |
| Iced | MIT | デスクトップ, Web (実験的) | 中 | Linux の COSMIC デスクトップで採用 |
| Slint | GPL-3.0（OSS 向け）/ ロイヤリティフリー（独自アプリのデスクトップ・モバイル・Web 向け）/ 商用（組込み含む） | デスクトップ, 組込み/MCU, Web, Android | 小〜大 | 組込み向け採用が強い。商用利用は条件を確認 |
| Dioxus | Apache-2.0 / MIT | デスクトップ, Web, モバイル, フルスタック | 小〜中 | 若く API が変化中 |
| Leptos | MIT | Web（CSR/SSR/ハイドレーション） | 小〜大 | ネイティブアプリには使えない |
| GTK4 | MIT (ラッパー) / LGPL-2.1+ (GTK 本体) | Linux が第一級、macOS/Windows も可 | 小〜大 | GNOME アプリで標準的 |

### 選定時のチェックリスト

1. **配布先**: ブラウザ → Leptos / デスクトップ+モバイル → Tauri, Dioxus / 組込み → Slint / Linux ネイティブ → GTK4
2. **チームのスキル**: Web 経験者 → Tauri, Dioxus, Leptos / 純 Rust → egui, Iced / デザイナー分業 → Slint, Tauri
3. **ライセンス**: Slint は商用利用の条件を事前確認。GTK は LGPL のリンク条件に注意。
4. **安定性の許容度**: 0.x のフレームワークは数か月ごとの API 追従コストを見込む。長期保守なら Tauri 2 / Slint 1 が有利。
5. **アクセシビリティ・IME・多言語入力**: ネイティブ/WebView 系が強く、独自描画系（egui, Iced, Slint）は要検証。日本語入力は特に実機で確認を。
6. **将来の保守**: 直近のリリース間隔と未解決 Issue 数、コミュニティ規模（上表）を確認。

## 7. 選び方の目安

- **既存の Web 資産がある / デザイン自由度が欲しい** → Tauri（または Dioxus）
- **とにかく速く小さなツール UI を作りたい** → egui
- **状態遷移を厳密に設計したい純 Rust アプリ** → Iced
- **UI と処理を分けたい / 組込みも視野** → Slint
- **React の感覚で Rust を書きたい** → Dioxus
- **Web アプリを Rust で書きたい** → Leptos
- **Linux デスクトップのネイティブアプリ** → GTK4

## 8. 補足

- 本比較は小さな TODO アプリ 1 つに基づく印象であり、大規模アプリでの優劣を示すものではありません。
- バージョンは 2026-10-10 時点の安定版です。Tauri 3、Dioxus 0.8、Leptos 0.9 などはアルファ/ベータで、API が変わる可能性があります。
- ビルド時間・サイズは環境と設定（`opt-level`、LTO、strip など）で大きく変わります。今回は既定のリリースプロファイルのままです。
