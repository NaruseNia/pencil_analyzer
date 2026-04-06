# pencil_analyzer

[日本語版 README はこちら](README_ja.md)

A CLI tool that parses Pencil ([pencil.dev](https://pencil.dev)) `.pen` files and outputs structural information in a format suitable for humans and AI.

## Purpose

Analyzes Pencil design files (`.pen`) to extract structure, components, and style information. Primarily designed to provide design context to AI, enabling text-based understanding of design contents.

## Installation

**Quick install** (Linux / macOS):

```bash
curl -fsSL https://raw.githubusercontent.com/NaruseNia/pencil_analyzer/main/scripts/install.sh | bash
```

You can specify a version or install directory:

```bash
# Install a specific version
curl -fsSL https://raw.githubusercontent.com/NaruseNia/pencil_analyzer/main/scripts/install.sh | bash -s 0.1.0

# Custom install directory
curl -fsSL https://raw.githubusercontent.com/NaruseNia/pencil_analyzer/main/scripts/install.sh | INSTALL_DIR=~/.local/bin bash
```

**Other methods**:

- Download a prebuilt binary from [Releases](https://github.com/NaruseNia/pencil_analyzer/releases)
- Build from source: `cargo install --path .`

## Usage

```bash
# Human-readable tree output (default)
pencil_analyzer design.pen

# JSON output
pencil_analyzer design.pen --format json

# Resolve component references (expand ref nodes)
pencil_analyzer design.pen --resolve-refs

# Resolve variables (uses default theme)
pencil_analyzer design.pen --resolve-vars

# Resolve variables with a specific theme
pencil_analyzer design.pen --resolve-vars --theme "mode=dark,spacing=condensed"

# Extract only components (reusable nodes)
pencil_analyzer design.pen --extract components

# Extract only variables
pencil_analyzer design.pen --extract variables

# Extract multiple categories
pencil_analyzer design.pen --extract components,variables,themes

# Filter output fields (show only content and fill)
pencil_analyzer design.pen --filter content,fill

# Filter by node type (only text nodes)
pencil_analyzer design.pen --type text

# Multiple types
pencil_analyzer design.pen --type frame,rectangle

# Combine extract + filter (components with content only)
pencil_analyzer design.pen --extract components --filter content

# Combine all: extract components, filter to text nodes, show only content
pencil_analyzer design.pen --extract components --type text --filter content

# Show pure hierarchy only (no properties)
pencil_analyzer design.pen --only-structure

# Limit tree depth (top-level only)
pencil_analyzer design.pen --depth 1

# Filter by regex on node path (name-based hierarchy)
pencil_analyzer design.pen --regex "Components/.*"
pencil_analyzer design.pen --regex ".*Button.*"

# List available values for --type, --filter, --extract
pencil_analyzer --list types

# Combine all options
pencil_analyzer design.pen --resolve-refs --resolve-vars --theme "mode=dark" --format json
```

### Options

| Flag | Description |
|---|---|
| `--format text\|json` | Output format (default: `text`) |
| `--resolve-refs` | Expand `ref` nodes into full component trees |
| `--resolve-vars` | Substitute `$variable` references with concrete values |
| `--theme <axes>` | Theme selection for variable resolution (e.g. `mode=dark`) |
| `--extract <categories>` | Extract specific categories: `components`, `variables`, `imports`, `themes` |
| `--type <types>` | Filter nodes by type: `rectangle`, `frame`, `text`, `ellipse`, `line`, `polygon`, `path`, `group`, `note`, `prompt`, `context`, `icon_font`, `ref` |
| `--filter <fields>` | Include only specified fields: `content`, `fill`, `layout`, `size`, `position`, `reusable`, `descendants`, `themes`, `variables`, `imports` |
| `--only-structure` | Show only the node hierarchy (type, id, name) without any properties |
| `--depth <n>` | Limit tree depth (`1` = top-level children only) |
| `--regex <pattern>` | Filter nodes by regex against their path (e.g. `Components/.*`) |
| `--list <category>` | List available values: `types`, `filters`, `extracts` |

### Output Examples

**Text format** — human-readable tree with types, IDs, sizes, and key properties:

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

**JSON format** — re-serialized document structure, suitable for piping to other tools or AI context.

## Supported .pen Features

- **Object types**: rectangle, frame, text, ellipse, line, polygon, path, group, note, prompt, context, icon_font, ref
- **Layout**: flexbox-style (vertical/horizontal, gap, padding, justifyContent, alignItems)
- **Graphics**: fill (color, gradient, image, mesh_gradient), stroke, effects (blur, shadow)
- **Components**: reusable components and ref instances with property overrides and descendant customization
- **Variables & Themes**: document-wide variables with theme-dependent values
- **Slots**: frame slots for container-style components

## Development

- **Language**: Rust (edition 2024)
- **Build tool**: Cargo

```bash
cargo build          # Debug build
cargo test           # Run tests (99 tests)
cargo clippy         # Lint
cargo fmt            # Format
```

### Release

```bash
./scripts/release.sh 0.1.0   # Update version, commit, tag
git push && git push origin v0.1.0  # Trigger CI release
```

## License

MIT License. See [LICENSE](LICENSE) for details.
