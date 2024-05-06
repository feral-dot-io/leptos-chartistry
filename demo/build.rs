use anyhow::{Context, Result};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_file, parsing::SyntaxSet};

fn main() -> Result<()> {
    // Find all of our examples
    println!("cargo:rerun-if-changed=src/examples");
    let in_examples = glob::glob("src/examples/*.rs")
        .context("invalid glob pattern")?
        .collect::<Result<Vec<_>, _>>()
        .context("failed to collect example paths")?;

    // Output directory
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    // Syntax highlighting
    syntax_hl_examples(&in_examples, out_dir.join("examples-hl"))
        .context("syntax higlighting examples")?;

    Ok(())
}

fn syntax_hl_examples(in_paths: &[PathBuf], out_dir: PathBuf) -> Result<()> {
    fs::create_dir_all(&out_dir).context("create out_dir")?;
    // Rust syntax highlighting settings
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    for src_path in in_paths {
        // Run highlighter
        let html = highlighted_html_for_file(src_path, &ss, theme)
            .context("failed to highlight example")?;
        // Write to file
        let mut out_path = out_dir.clone();
        out_path.push(src_path.file_name().unwrap());
        let mut file = File::create(out_path).context("create highlight example file")?;
        file.write_all(html.as_bytes())
            .context("writing highlight example")?;
    }
    Ok(())
}
