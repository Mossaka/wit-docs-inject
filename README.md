# wit-docs-inject

A toolkit for injecting and viewing WIT documentation in WebAssembly components.

## Tools

This project provides two complementary tools:

### wit-docs-inject
Injects `package-docs` from WIT source files into WebAssembly components.

### wit-docs-view  
Views documentation from WebAssembly components that have been processed with `wit-docs-inject`.

## Quick Start

1. **Build the tools:**
   ```bash
   cargo build --release
   ```

2. **Inject documentation into a component:**
   ```bash
   ./target/release/wit-docs-inject --component my-component.wasm --wit-dir ./wit
   ```

3. **View the documentation:**
   ```bash
   ./target/release/wit-docs-view my-component.docs.wasm
   ```

## wit-docs-inject Usage

```bash
# Create a new component with docs (default: adds .docs.wasm suffix)
wit-docs-inject --component component.wasm --wit-dir wit-source/

# Overwrite the original component in-place
wit-docs-inject --component component.wasm --wit-dir wit-source/ --inplace

# Specify custom output path
wit-docs-inject --component component.wasm --wit-dir wit-source/ --out documented-component.wasm
```

### Options

- `--component <COMPONENT>`: Input component (.wasm) path
- `--wit-dir <WIT_DIR>`: WIT package directory whose docstrings you want to embed
- `--out <OUT>`: Output component path (default: adds .docs.wasm suffix)
- `--inplace`: Overwrite the input file in place

## wit-docs-view Usage

```bash
# View documentation in pretty format (default)
wit-docs-view component.wasm

# View documentation in JSON format
wit-docs-view component.wasm --format json

# View documentation in Markdown format
wit-docs-view component.wasm --format markdown

# View documentation as WIT with integrated docs
wit-docs-view component.wasm --format wit

# Show only function documentation
wit-docs-view component.wasm --functions-only

# Show only world documentation
wit-docs-view component.wasm --worlds-only

# Extract complete WIT with docs and save to file
wit-docs-view component.wasm --format wit > component-with-docs.wit
```

### Output Formats

#### Pretty (Default)
```
🌍 World: fetch
   📝 An example world for the component to target.

📤 Exported Functions:
   🔧 fetch: Fetch the webpage
```

#### JSON
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

#### Markdown

```markdown
# World: fetch

An example world for the component to target.

## Exported Functions

### `fetch`
Fetch the webpage
```

#### WIT (New!)

The `--format wit` option reconstructs the WIT interface definition with the original documentation comments integrated back in:

```wit
package root:component;

/// An example world for the component to target.
world root {
  import wasi:io/poll@0.2.0;
  import wasi:clocks/monotonic-clock@0.2.0;
  // ... other imports ...
  
  /// Fetch the webpage
  export fetch: func(url: string) -> result<string, string>;
}
```

This allows you to:
- Extract a complete WIT file with documentation from a compiled component
- Bridge the gap between compiled components and source WIT files
- Preserve documentation throughout the development lifecycle

## How It Works

1. **wit-docs-inject** extracts documentation from WIT source files and embeds it as a `package-docs` custom section in the WebAssembly component
2. **wit-docs-view** reads the `package-docs` custom section and displays the documentation in various formats
3. The documentation is stored as structured JSON metadata, making it accessible to documentation tools and IDEs

## Installation

```bash
# Build release binaries
cargo build --release

# Optionally, install to cargo bin directory
cargo install --path .
```

## Example Workflow

```bash
# Start with a component that has no docs
wit-docs-view my-component.wasm
# Output: No package-docs found in component

# Inject docs from WIT source
wit-docs-inject --component my-component.wasm --wit-dir ./wit

# Now view the documentation
wit-docs-view my-component.docs.wasm
# Output: Beautiful formatted documentation!
```

## Exit Codes

Both tools use standard exit codes:
- `0`: Success
- `1`: Error (with descriptive error message)
