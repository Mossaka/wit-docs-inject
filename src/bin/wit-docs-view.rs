use anyhow::{Context, Result};
use clap::Parser;
use serde_json::Value;
use std::{fs, path::PathBuf, process::Command};
use wasmparser::{Parser as WasmParser, Payload};

/// View documentation from a WebAssembly component's `package-docs` custom section.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the WebAssembly component (.wasm) file
    component: PathBuf,

    /// Output format
    #[arg(long, value_enum, default_value = "pretty")]
    format: OutputFormat,

    /// Show only function documentation
    #[arg(long)]
    functions_only: bool,

    /// Show only world documentation
    #[arg(long)]
    worlds_only: bool,
}

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
enum OutputFormat {
    Pretty,
    Json,
    Markdown,
    Wit,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let wasm_bytes = fs::read(&args.component)
        .with_context(|| format!("Failed to read component file: {:?}", args.component))?;

    let docs_json = extract_package_docs(&wasm_bytes)
        .with_context(|| "Failed to extract package-docs from component")?;

    if let Some(docs) = docs_json {
        display_docs(&docs, &args)?;
    } else {
        eprintln!("No package-docs found in component");
        std::process::exit(1);
    }

    Ok(())
}

fn extract_package_docs(wasm_bytes: &[u8]) -> Result<Option<Value>> {
    let parser = WasmParser::new(0);
    
    for payload in parser.parse_all(wasm_bytes) {
        let payload = payload.context("Failed to parse WebAssembly")?;
        
        if let Payload::CustomSection(reader) = payload {
            if reader.name() == "package-docs" {
                let data = reader.data();
                // Skip the first byte (version) and parse the JSON
                if data.len() > 1 {
                    let json_data = &data[1..];
                    let docs: Value = serde_json::from_slice(json_data)
                        .context("Failed to parse package-docs JSON")?;
                    return Ok(Some(docs));
                }
            }
        }
    }
    
    Ok(None)
}

fn display_docs(docs: &Value, args: &Args) -> Result<()> {
    match args.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(docs)?);
        }
        OutputFormat::Pretty => {
            display_pretty(docs, args)?;
        }
        OutputFormat::Markdown => {
            display_markdown(docs, args)?;
        }
        OutputFormat::Wit => {
            display_wit_with_docs(docs, args)?;
        }
    }
    Ok(())
}

fn display_pretty(docs: &Value, args: &Args) -> Result<()> {
    if let Some(worlds) = docs.get("worlds").and_then(|w| w.as_object()) {
        for (world_name, world_data) in worlds {
            if !args.functions_only {
                println!("🌍 World: {}", world_name);
                
                if let Some(world_docs) = world_data.get("docs").and_then(|d| d.as_str()) {
                    println!("   📝 {}", world_docs);
                } else {
                    println!("   📝 (no documentation)");
                }
                println!();
            }

            if !args.worlds_only {
                if let Some(func_exports) = world_data.get("func_exports").and_then(|f| f.as_object()) {
                    if !func_exports.is_empty() {
                        if !args.functions_only {
                            println!("📤 Exported Functions:");
                        }
                        
                        for (func_name, func_data) in func_exports {
                            print!("   🔧 {}", func_name);
                            
                            if let Some(func_docs) = func_data.get("docs").and_then(|d| d.as_str()) {
                                println!(": {}", func_docs);
                            } else {
                                println!(": (no documentation)");
                            }
                        }
                        println!();
                    }
                }

                if let Some(func_imports) = world_data.get("func_imports").and_then(|f| f.as_object()) {
                    if !func_imports.is_empty() {
                        if !args.functions_only {
                            println!("📥 Imported Functions:");
                        }
                        
                        for (func_name, func_data) in func_imports {
                            print!("   🔧 {}", func_name);
                            
                            if let Some(func_docs) = func_data.get("docs").and_then(|d| d.as_str()) {
                                println!(": {}", func_docs);
                            } else {
                                println!(": (no documentation)");
                            }
                        }
                        println!();
                    }
                }
            }
        }
    } else {
        println!("No world documentation found");
    }
    
    Ok(())
}

fn display_markdown(docs: &Value, args: &Args) -> Result<()> {
    if let Some(worlds) = docs.get("worlds").and_then(|w| w.as_object()) {
        for (world_name, world_data) in worlds {
            if !args.functions_only {
                println!("# World: {}", world_name);
                println!();
                
                if let Some(world_docs) = world_data.get("docs").and_then(|d| d.as_str()) {
                    println!("{}", world_docs);
                } else {
                    println!("*(no documentation)*");
                }
                println!();
            }

            if !args.worlds_only {
                if let Some(func_exports) = world_data.get("func_exports").and_then(|f| f.as_object()) {
                    if !func_exports.is_empty() {
                        if !args.functions_only {
                            println!("## Exported Functions");
                            println!();
                        }
                        
                        for (func_name, func_data) in func_exports {
                            println!("### `{}`", func_name);
                            
                            if let Some(func_docs) = func_data.get("docs").and_then(|d| d.as_str()) {
                                println!("{}", func_docs);
                            } else {
                                println!("*(no documentation)*");
                            }
                            println!();
                        }
                    }
                }

                if let Some(func_imports) = world_data.get("func_imports").and_then(|f| f.as_object()) {
                    if !func_imports.is_empty() {
                        if !args.functions_only {
                            println!("## Imported Functions");
                            println!();
                        }
                        
                        for (func_name, func_data) in func_imports {
                            println!("### `{}`", func_name);
                            
                            if let Some(func_docs) = func_data.get("docs").and_then(|d| d.as_str()) {
                                println!("{}", func_docs);
                            } else {
                                println!("*(no documentation)*");
                            }
                            println!();
                        }
                    }
                }
            }
        }
    } else {
        println!("No world documentation found");
    }
    
    Ok(())
}

fn display_wit_with_docs(docs: &Value, args: &Args) -> Result<()> {
    // First, get the original WIT text from the component
    let output = Command::new("wasm-tools")
        .args(&["component", "wit", &args.component.to_string_lossy()])
        .output()
        .context("Failed to run wasm-tools component wit")?;
    
    if !output.status.success() {
        anyhow::bail!("wasm-tools component wit failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    let wit_text = String::from_utf8(output.stdout)
        .context("Failed to parse wasm-tools output as UTF-8")?;
    
    // Parse and inject documentation
    let enhanced_wit = inject_docs_into_wit(&wit_text, docs)?;
    println!("{}", enhanced_wit);
    
    Ok(())
}

fn inject_docs_into_wit(wit_text: &str, docs: &Value) -> Result<String> {
    let mut result = String::new();
    let lines: Vec<&str> = wit_text.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        // Look for world definitions
        if line.starts_with("world ") {
            let world_name = extract_world_name(line);
            
            // Add world documentation before the world declaration
            if let Some(world_docs) = get_world_docs(docs, &world_name) {
                for doc_line in world_docs.lines() {
                    result.push_str(&format!("/// {}\n", doc_line));
                }
            }
            
            result.push_str(lines[i]);
            result.push('\n');
            i += 1;
            
            // Process the world body
            while i < lines.len() {
                let current_line = lines[i];
                let trimmed = current_line.trim();
                
                // Check if this is an export/import function
                if trimmed.starts_with("export ") || trimmed.starts_with("import ") {
                    if let Some(func_name) = extract_function_name(trimmed) {
                        // Add function documentation before the function declaration
                        if let Some(func_docs) = get_function_docs(docs, &world_name, &func_name) {
                            let indent = get_indent(current_line);
                            for doc_line in func_docs.lines() {
                                result.push_str(&format!("{}/// {}\n", indent, doc_line));
                            }
                        }
                    }
                }
                
                result.push_str(current_line);
                result.push('\n');
                i += 1;
                
                // Stop when we reach the end of the world
                if trimmed == "}" {
                    break;
                }
            }
        } else {
            result.push_str(lines[i]);
            result.push('\n');
            i += 1;
        }
    }
    
    Ok(result)
}

fn extract_world_name(line: &str) -> String {
    // Extract world name from "world <name> {" pattern
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        parts[1].to_string()
    } else {
        "unknown".to_string()
    }
}

fn extract_function_name(line: &str) -> Option<String> {
    // Extract function name from "export/import <name>: func(...)" pattern
    if let Some(colon_pos) = line.find(':') {
        let before_colon = &line[..colon_pos];
        let parts: Vec<&str> = before_colon.split_whitespace().collect();
        if parts.len() >= 2 {
            return Some(parts[1].to_string());
        }
    }
    None
}

fn get_indent(line: &str) -> &str {
    let trimmed_len = line.trim_start().len();
    &line[..line.len() - trimmed_len]
}

fn get_world_docs(docs: &Value, world_name: &str) -> Option<String> {
    let worlds = docs.get("worlds").and_then(|w| w.as_object())?;
    
    // First try exact match
    if let Some(world) = worlds.get(world_name) {
        return world.get("docs").and_then(|d| d.as_str()).map(|s| s.to_string());
    }
    
    // If no exact match and there's only one world, use that
    if worlds.len() == 1 {
        let (_, world_data) = worlds.iter().next().unwrap();
        return world_data.get("docs").and_then(|d| d.as_str()).map(|s| s.to_string());
    }
    
    None
}

fn get_function_docs(docs: &Value, world_name: &str, func_name: &str) -> Option<String> {
    let worlds = docs.get("worlds").and_then(|w| w.as_object())?;
    
    // First try exact world match
    if let Some(world) = worlds.get(world_name) {
        return get_function_docs_from_world(world, func_name);
    }
    
    // If no exact match and there's only one world, use that
    if worlds.len() == 1 {
        let (_, world_data) = worlds.iter().next().unwrap();
        return get_function_docs_from_world(world_data, func_name);
    }
    
    None
}

fn get_function_docs_from_world(world: &Value, func_name: &str) -> Option<String> {
    // Try both func_exports and functions for backward compatibility
    world.get("func_exports")
        .or_else(|| world.get("functions"))
        .and_then(|funcs| funcs.as_object())
        .and_then(|functions| functions.get(func_name))
        .and_then(|func| func.get("docs"))
        .and_then(|d| d.as_str())
        .map(|s| s.to_string())
}
