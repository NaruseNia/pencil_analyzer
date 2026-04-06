---
name: pencil-analyzer
description: |
  Use pencil_analyzer CLI to parse, analyze, and extract structural information from Pencil (.pen) design files.
  Trigger this skill when the user wants to: review or understand a .pen file's structure, extract components/variables/themes from a design,
  get layout or style information from a .pen file, prepare design context for AI consumption, inspect specific node types or patterns in a design,
  or compare design structure before/after changes. Also trigger when the user mentions "pencil_analyzer", "pen file analysis",
  or asks about design structure, components, or styles in the context of .pen files.
---

# pencil-analyzer

Analyze Pencil (.pen) design files using the `pencil_analyzer` CLI to extract structure, components, styles, and other design information.

## When to use this skill

- Reviewing or understanding a `.pen` file's design structure
- Extracting reusable components, variables, themes, or imports
- Getting specific information (layout, fills, text content, sizes, positions)
- Preparing design context as input for AI tasks
- Inspecting specific node types (frames, text, rectangles, refs, etc.)
- Filtering nodes by name or path patterns
- Checking design structure after Pencil MCP edits

## Running pencil_analyzer

Always use `npx` to run the latest version:

```bash
npx @narusenia/pencil-analyzer@latest <file.pen> [options]
```

## Core options

### Output format

- `--format text` (default): Human-readable indented tree. Best for reviewing structure at a glance.
- `--format json`: Machine-readable JSON. Best when you need to parse or process the output programmatically.

### Resolving references and variables

- `--resolve-refs`: Expand `ref` (component instance) nodes into their full component definitions. Use this when you need to see what a component instance actually contains rather than just its reference ID.
- `--resolve-vars`: Substitute `$variable` references with their concrete values. Without this, variable-driven properties show as `$variableName`.
- `--theme "axis=value,axis2=value2"`: Select a specific theme when resolving variables. Example: `--theme "mode=dark,spacing=condensed"`. If omitted, the default theme (first value of each axis) is used.

### Filtering what to show

- `--filter <fields>`: Comma-separated list of property categories to include in output. Omitting this shows everything.
  - `content` - Text content
  - `fill` - Fill colors, gradients, images
  - `layout` - Flexbox-style layout (direction, justify, align, padding, gap)
  - `size` - Width and height
  - `position` - X/Y coordinates
  - `reusable` - Whether a node is a reusable component
  - `descendants` - Ref descendant overrides
  - `themes` - Document theme definitions
  - `variables` - Document variable definitions
  - `imports` - Import declarations

- `--only-structure`: Show only the node hierarchy (type, id, name) with no properties at all. Useful for getting a quick structural overview.

- `--depth <n>`: Limit tree traversal depth. `--depth 1` shows only top-level children. Useful for large files where you want a high-level view first.

### Extracting specific categories

- `--extract <categories>`: Extract only specific document-level categories (comma-separated).
  - `components` - Nodes marked as reusable
  - `variables` - Variable definitions
  - `imports` - Import declarations
  - `themes` - Theme axis definitions

### Filtering by node type or pattern

- `--type <types>`: Comma-separated node types to include: `rectangle`, `frame`, `text`, `ellipse`, `line`, `polygon`, `path`, `group`, `note`, `prompt`, `context`, `icon_font`, `ref`
- `--regex <pattern>`: Filter nodes by regex against their hierarchical path (e.g., `".*Button.*"`, `"Components/.*"`, `"Header/Nav/.*"`). The path is built from node names separated by `/`.

## Efficiency: one command per task

Token and time costs matter. Prefer a single, well-targeted command over multiple exploratory ones. Combine flags in one call to get exactly the information needed.

### Command selection guide

| User intent | Command |
|---|---|
| Overall structure | `--only-structure` (add `--depth N` for very large files only) |
| List components | `--extract components` |
| List variables/themes | `--extract variables,themes` |
| Expand a ref instance | `--resolve-refs --regex ".*Name.*"` |
| Text contents only | `--type text --filter content` |
| Layout and sizing | `--filter layout,size` |
| Dark theme values | `--resolve-vars --theme "mode=dark"` |
| Full JSON for processing | `--resolve-refs --resolve-vars --format json` |
| Specific area details | `--regex "Header/.*"` |

Only run a second command if the first output is insufficient or the user asks for more detail. Do not pre-emptively run overview + detail as two separate calls.

## Integration with Pencil MCP tools

pencil_analyzer and Pencil MCP tools serve complementary roles:

- **Pencil MCP tools** (`batch_get`, `batch_design`, etc.): Read and write individual nodes interactively within the editor. Use these for making changes, getting screenshots, or working with the live editor state.
- **pencil_analyzer CLI**: Parse the entire `.pen` file on disk and output a structured overview. Use this for broad analysis, reviews, extracting patterns across the whole file, or preparing context summaries.

**Typical workflow:**
1. Use `pencil_analyzer` to understand the overall structure and identify areas of interest
2. Use Pencil MCP `batch_get` to inspect specific nodes in detail
3. Use Pencil MCP `batch_design` to make changes
4. Use `pencil_analyzer` again to verify the result or generate a summary

## Tips

- Combine flags aggressively to get what you need in one call (e.g., `--resolve-refs --type text --filter content`).
- Use `--filter` to reduce output size when you only care about specific properties.
- Use `--depth N` only for genuinely large files (100+ nodes). For small/medium files, the default full output is fine.
- The `--list types|filters|extracts` flag (without a file) shows all available values for those options.
