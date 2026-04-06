# pencil_analyzer

[English README](README.md)

Pencil ([pencil.dev](https://pencil.dev)) の `.pen` ファイル構造を解析し、人間やAIが利用しやすい形式で情報を出力するCLIツールです。

## 目的

Pencilのデザインファイル (`.pen`) を解析し、デザインの構造・コンポーネント・スタイル情報などを抽出します。特にAIへのコンテキストとして渡すことを想定しており、デザインの内容をテキストベースで把握できるようにすることを目指しています。

## 開発環境

- **言語**: Rust (edition 2024)
- **ビルドツール**: Cargo

## セットアップ

```bash
# ビルド
cargo build

# 実行
cargo run

# リリースビルド
cargo build --release
```

## ステータス

現在開発初期段階です。

## ライセンス

TBD
