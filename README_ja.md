# pencil_analyzer

[English README](README.md)

Pencil ([pencil.dev](https://pencil.dev)) の `.pen` ファイル構造を解析し、人間やAIが利用しやすい形式で情報を出力するCLIツールです。

## 目的

Pencilのデザインファイル (`.pen`) を解析し、デザインの構造・コンポーネント・スタイル情報などを抽出します。特にAIへのコンテキストとして渡すことを想定しており、デザインの内容をテキストベースで把握できるようにすることを目指しています。

## インストール

**ワンライナーインストール** (Linux / macOS):

```bash
curl -fsSL https://raw.githubusercontent.com/NaruseNia/pencil_analyzer/main/scripts/install.sh | bash
```

バージョンやインストール先を指定することもできます:

```bash
# バージョン指定
curl -fsSL https://raw.githubusercontent.com/NaruseNia/pencil_analyzer/main/scripts/install.sh | bash -s 0.1.0

# インストール先を変更
curl -fsSL https://raw.githubusercontent.com/NaruseNia/pencil_analyzer/main/scripts/install.sh | INSTALL_DIR=~/.local/bin bash
```

**その他の方法**:

- [Releases](https://github.com/NaruseNia/pencil_analyzer/releases) からビルド済みバイナリをダウンロード
- ソースからビルド: `cargo install --path .`

## 使い方

```bash
# 人間可読なツリー表示 (デフォルト)
pencil_analyzer design.pen

# JSON出力
pencil_analyzer design.pen --format json

# コンポーネント参照を展開 (refノードを解決)
pencil_analyzer design.pen --resolve-refs

# 変数を解決 (デフォルトテーマ使用)
pencil_analyzer design.pen --resolve-vars

# テーマを指定して変数を解決
pencil_analyzer design.pen --resolve-vars --theme "mode=dark,spacing=condensed"

# コンポーネント (再利用可能ノード) だけ抽出
pencil_analyzer design.pen --extract components

# 変数定義だけ抽出
pencil_analyzer design.pen --extract variables

# 複数カテゴリを指定
pencil_analyzer design.pen --extract components,variables,themes

# 出力フィールドを絞り込み (contentとfillのみ)
pencil_analyzer design.pen --filter content,fill

# ノードタイプで絞り込み (テキストノードのみ)
pencil_analyzer design.pen --type text

# 複数タイプ指定
pencil_analyzer design.pen --type frame,rectangle

# extract + filter の組み合わせ (コンポーネントのcontentだけ)
pencil_analyzer design.pen --extract components --filter content

# 全組み合わせ: コンポーネントからtextノードだけ抽出し、contentのみ表示
pencil_analyzer design.pen --extract components --type text --filter content

# 全オプションを組み合わせ
pencil_analyzer design.pen --resolve-refs --resolve-vars --theme "mode=dark" --format json
```

### オプション

| フラグ | 説明 |
|---|---|
| `--format text\|json` | 出力形式 (デフォルト: `text`) |
| `--resolve-refs` | `ref` ノードを完全なコンポーネントツリーに展開 |
| `--resolve-vars` | `$variable` 参照を具体的な値に置換 |
| `--theme <axes>` | 変数解決のテーマ指定 (例: `mode=dark`) |
| `--extract <categories>` | 特定カテゴリを抽出: `components`, `variables`, `imports`, `themes` |
| `--type <types>` | ノードタイプで絞り込み: `rectangle`, `frame`, `text`, `ellipse`, `line`, `polygon`, `path`, `group`, `note`, `prompt`, `context`, `icon_font`, `ref` |
| `--filter <fields>` | 指定フィールドのみ出力: `content`, `fill`, `layout`, `size`, `position`, `reusable`, `descendants`, `themes`, `variables`, `imports` |

### 出力例

**テキスト形式** — 型、ID、サイズ、主要プロパティを含む人間可読なツリー:

```
Document (version 2.9)
  Themes: mode[light, dark]
  Variables: 2 defined
    $color.background: color
    $color.text: color

  [frame] round-button (200x48) @(0,0)
    reusable: true
    fill: Color(Value("#3B82F6"))
    layout: Horizontal
    [text] label
      fill: Color(Value("#FFFFFF"))
      content: Submit
  [frame] landing-page (1440x900) @(300,0)
    fill: Color(Variable("$color.background"))
    layout: Vertical
    [text] hero-title
      content: Welcome to Pencil
    [ref] cta-button -> round-button
      override: label
```

**JSON形式** — 再シリアライズされたドキュメント構造。他ツールやAIコンテキストへのパイプに最適。

## 対応する .pen 機能

- **オブジェクト型**: rectangle, frame, text, ellipse, line, polygon, path, group, note, prompt, context, icon_font, ref
- **レイアウト**: flexboxスタイル (vertical/horizontal, gap, padding, justifyContent, alignItems)
- **グラフィックス**: fill (color, gradient, image, mesh_gradient), stroke, effects (blur, shadow)
- **コンポーネント**: 再利用可能コンポーネントとrefインスタンス (プロパティオーバーライド、descendantsカスタマイズ)
- **変数 & テーマ**: ドキュメント全体の変数、テーマ依存値
- **スロット**: コンテナ型コンポーネントのフレームスロット

## 開発

- **言語**: Rust (edition 2024)
- **ビルドツール**: Cargo

```bash
cargo build          # デバッグビルド
cargo test           # テスト実行 (88テスト)
cargo clippy         # Lint
cargo fmt            # フォーマット
```

### リリース

```bash
./scripts/release.sh 0.1.0   # バージョン更新、コミット、タグ作成
git push && git push origin v0.1.0  # CIリリースをトリガー
```

## ライセンス

TBD
