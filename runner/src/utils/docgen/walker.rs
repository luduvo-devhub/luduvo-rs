use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::utils::docgen::markdown::write_markdown;
use crate::utils::docgen::parser::parse_file;

pub fn process_crates(crates_dir: &Path, output_dir: &Path) -> Result<()> {
    for entry in fs::read_dir(crates_dir)? {
        let entry = entry?;
        let crate_path = entry.path();

        if !crate_path.is_dir() {
            continue;
        }

        let crate_name = crate_path.file_name().unwrap();
        let src_path = crate_path.join("src");

        if !src_path.exists() {
            continue;
        }

        let crate_out = output_dir.join(crate_name);
        fs::create_dir_all(&crate_out)?;

        process_src(&src_path, &crate_out)?;
    }

    Ok(())
}

fn process_src(src: &Path, out: &Path) -> Result<()> {
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if path.extension().map(|e| e == "rs").unwrap_or(false) {
            let module_path = derive_module_path(src, path)?;
            let mut out_dir = out.join(module_path);

            std::fs::create_dir_all(&out_dir)?;

            let items = parse_file(path)?;

            for item in items {
                write_markdown(&mut out_dir, &item)?;
            }
        }
    }

    Ok(())
}

fn derive_module_path(src: &Path, file: &Path) -> Result<std::path::PathBuf> {
    let relative = file.strip_prefix(src)?;

    let mut components: Vec<_> = relative.components().collect();

    // Remove filename
    let file_name = components.pop().unwrap().as_os_str().to_string_lossy();

    let mut path = std::path::PathBuf::new();

    for comp in components {
        path.push(comp.as_os_str());
    }

    // Handle file name
    let file_stem = file_name.strip_suffix(".rs").unwrap();

    // Skip lib.rs and mod.rs as folders
    if file_stem != "lib" && file_stem != "mod" {
        path.push(file_stem);
    }

    Ok(path)
}
