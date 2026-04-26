pub mod markdown;
pub mod parser;
pub mod walker;

use anyhow::Result;
use std::fs;
use std::path::Path;

use walker::process_crates;

pub fn generate_docs(crates_dir: &str, output_dir: &str) -> Result<()> {
    let crates_path = Path::new(crates_dir);
    let output_path = Path::new(output_dir);

    if output_path.exists() {
        fs::remove_dir_all(output_path)?;
    }

    fs::create_dir_all(output_path)?;

    process_crates(crates_path, output_path)?;

    Ok(())
}
