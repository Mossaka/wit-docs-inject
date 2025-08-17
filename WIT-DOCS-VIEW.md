# wit-docs-view

A command-line utility to view documentation from WebAssembly components that have been processed with `wit-docs-inject`.

## Usage

```bash
# View documentation in pretty format (default)
wit-docs-view component.wasm

# View documentation in JSON format
wit-docs-view component.wasm --format json

# View documentation in Markdown format
wit-docs-view component.wasm --format markdown

# Show only function documentation
wit-docs-view component.wasm --functions-only

# Show only world documentation
wit-docs-view component.wasm --worlds-only
```

## Output Formats

### Pretty (Default)
Human-readable output with emojis and formatting:
```
🌍 World: fetch
   📝 An example world for the component to target.

📤 Exported Functions:
   🔧 fetch: Fetch the webpage
```

### JSON
Raw JSON structure for programmatic use:
```json
{
  "worlds": {
    "fetch": {
      "docs": "An example world for the component to target.",
      "func_exports": {
        "fetch": {
          "docs": "Fetch the webpage"
        }
      }
    }
  }
}
```

### Markdown
Markdown format suitable for documentation:
```markdown
# World: fetch

An example world for the component to target.

## Exported Functions

### `fetch`
Fetch the webpage
```

## Exit Codes

- `0`: Success - documentation found and displayed
- `1`: No package-docs found in component

## Building

```bash
cargo build --release
```

The binary will be available at `target/release/wit-docs-view`.
