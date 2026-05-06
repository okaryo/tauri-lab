# tauri-lab

Tauri + Svelte + TypeScript を使ったデスクトップアプリの練習用プロジェクトです。

このリポジトリは、本番用アプリケーションを作る前の予行演習として使います。
目的はアプリケーションを素早く完成させることではなく、Tauri と Rust の基本を押さえながら、デスクトップアプリ開発で必要になる要素を小さく試すことです。

## 作りたいアプリの方向性

本番で作りたいアプリケーションでは、以下のような機能を想定しています。

- 作業ログ・todo の作成と管理
- SQLite または JSON を使ったローカルデータ管理
- ポモドーロタイマー
- タイマー完了や作業ログ記入を促す通知
- グローバルホットキー
- 設定した時間ごとに作業ログの記入を促す仕組み
- ポモドーロ中に環境音を流す機能
- ポモドーロ中に制限するアプリやブラウザ URL の設定

この練習用アプリでは、これらを本格実装する前に、Tauri / Rust 側で必要になる技術要素を段階的に検証します。

## ハンズオンの進め方

各ステップは以下の流れで進めます。

- [ ] その回のゴールを決める
- [ ] 背景知識を短く確認する
- [ ] 小さく実装する
- [ ] 動かして確認する
- [ ] 学んだことを整理する
- [ ] 次へ進む前に質問や疑問点を確認する

特に重視することは、Svelte から Rust を呼び出す境界、Rust 側の型設計、エラー処理、データ永続化、Tauri plugin と権限管理を理解することです。

## 学習ロードマップ

### Step 0: セットアップ

- [x] Tauri + Svelte + TypeScript の最小構成を作る
- [x] pnpm を使う構成にする
- [x] Tauri 用アイコンを生成する
- [x] `pnpm run check` を通す
- [x] `pnpm run build` を通す
- [x] `cargo check` を通す
- [x] `pnpm run tauri build` を通す

### Phase 1: Tauri の基本

- [ ] Svelte から Rust command を呼ぶ
- [ ] Rust から値を返す
- [ ] Rust command で引数を受け取る
- [ ] `Result<T, E>` を使ってエラーを返す
- [ ] `serde` を使って struct をやり取りする

### Phase 2: Todo / WorkLog の最小 CRUD

- [ ] Rust に `Todo` / `WorkLog` の struct を作る
- [ ] `create_todo` を作る
- [ ] `list_todos` を作る
- [ ] `complete_todo` を作る
- [ ] まずはメモリ上で管理する
- [ ] その後 JSON 保存に進む

最初の具体的なゴールは、Svelte UI から Rust command を呼び、Todo を作成・一覧表示・完了できるようにすることです。

### Phase 3: 永続化

- [ ] アプリデータディレクトリの扱いを知る
- [ ] JSON ファイルへ保存する
- [ ] JSON ファイルから読み込む
- [ ] SQLite を使って保存する
- [ ] JSON と SQLite の使い分けを理解する

作業ログや todo のように検索・集計が増えそうなデータは SQLite が向いています。
一方で、アプリ設定のような小さなデータは JSON や Tauri store plugin が向いています。

### Phase 4: Tauri plugin と権限

- [ ] Notification plugin を使う
- [ ] Global Shortcut plugin を使う
- [ ] `src-tauri/capabilities/default.json` に権限を追加する
- [ ] フロントエンドに許可する権限の考え方を理解する

Tauri v2 では、フロントエンドから使える機能を capabilities で明示的に許可します。
通知やホットキーは、この権限管理を学ぶ題材として扱います。

### Phase 5: 実アプリ寄りの機能

- [ ] ポモドーロタイマーを作る
- [ ] タイマー完了時に通知する
- [ ] タイマー状態を保存・復元する
- [ ] 設定画面を作る
- [ ] 環境音を再生する
- [ ] アプリやブラウザ URL 制限の実現方法を調査する

アプリやブラウザ URL の制限は OS やブラウザごとの差が大きく、権限や配布時の説明も重くなります。
この練習用アプリでは、まずは調査と小さな検証に留め、実際のブロック機能を急いで作り込まない方針です。

## 構成

- `src/`: Svelte で書くフロントエンド
- `src-tauri/`: Rust で書く Tauri アプリケーション本体
- `src-tauri/src/lib.rs`: フロントエンドから呼び出す Rust コマンド
- `src-tauri/tauri.conf.json`: ウィンドウ、ビルド、アプリ識別子などの Tauri 設定
- `src-tauri/capabilities/`: フロントエンドに許可する Tauri 権限の設定

## セットアップ

依存関係をインストールします。

```sh
pnpm install
```

開発用に起動します。

```sh
pnpm run tauri dev
```

フロントエンドだけを確認したい場合は、Vite の開発サーバーだけを起動できます。

```sh
pnpm run dev
```
