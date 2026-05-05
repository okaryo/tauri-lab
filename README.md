# tauri-lab

Tauri + Svelte + TypeScript を使ったデスクトップアプリの練習用プロジェクトです。

## 構成

- `src/`: Svelte で書くフロントエンド
- `src-tauri/`: Rust で書く Tauri アプリケーション本体
- `src-tauri/src/lib.rs`: フロントエンドから呼び出す Rust コマンド
- `src-tauri/tauri.conf.json`: ウィンドウ、ビルド、アプリ識別子などの Tauri 設定

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
