use anyhow::{Context, Result};
use clap::Parser;
use std::{borrow::Cow, fs, path::PathBuf};
use wasm_encoder::{Component, CustomSection};
use wasm_encoder::reencode::RoundtripReencoder;
use wasm_encoder::reencode::component_utils::parse_component;
use wit_parser::{PackageMetadata, Resolve};

/// Inject `package-docs` from a .wit source dir into a component.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input component (.wasm) path
    #[arg(long)]
    component: PathBuf,

    /// WIT package dir whose docstrings you want to embed
    #[arg(long)]
    wit_dir: PathBuf,

    /// Output component path (default: in-place overwrite disabled; write alongside with .docs.wasm)
    #[arg(long)]
    out: Option<PathBuf>,

    /// Overwrite the input file in place
    #[arg(long, default_value_t = false)]
    inplace: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = fs::read(&args.component)
        .with_context(|| format!("reading {:?}", args.component))?;

    // 1) Build WIT docs -> binary metadata payload ("package-docs")
    let mut resolve = Resolve::new();
    let (pkg_id, _sources) = resolve
        .push_dir(&args.wit_dir)
        .with_context(|| format!("parsing WIT dir {:?}", args.wit_dir))?;

    // Extract doc metadata from the WIT package and encode to bytes
    let meta = PackageMetadata::extract(&resolve, pkg_id);
    let payload = meta.encode().context("encoding package-docs")?;

    // 2) Reencode component verbatim and append our custom section
    let mut out_comp = Component::new();

    // Round-trip copy all existing sections exactly.
    // (This preserves ordering/contents; we only add one extra custom section at the end.)
    let mut rr = RoundtripReencoder;
    let parser = wasmparser::Parser::new(0);
    parse_component(&mut rr, &mut out_comp, parser, &input, &input)
        .context("reencoding original component")?;

    // Append `package-docs` custom section for components.
    // Note: SECTION_NAME is "package-docs".
    let section = CustomSection {
        name: Cow::Borrowed(PackageMetadata::SECTION_NAME),
        data: Cow::Owned(payload),
    };
    out_comp.section(&section);

    let bytes = out_comp.finish();

    // 3) Write output
    let out_path = if args.inplace {
        args.component.clone()
    } else if let Some(out) = args.out {
        out
    } else {
        let mut p = args.component.clone();
        let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext.is_empty() { p.set_extension("wasm"); }
        let stem = p.file_stem().unwrap_or_default().to_string_lossy();
        let parent = p.parent().unwrap_or_else(|| std::path::Path::new("."));
        let mut out = parent.join(format!("{stem}.docs.wasm"));
        // avoid the case where `component` had no ext and we changed it above
        if out == args.component { out = parent.join(format!("{stem}.docs.injected.wasm")); }
        out
    };
    fs::write(&out_path, bytes).with_context(|| format!("writing {:?}", out_path))?;

    eprintln!("Injected package-docs into {:?}", out_path);
    Ok(())
}
