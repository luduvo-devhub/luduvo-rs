use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use super::parser::DocItem;

pub fn write_markdown(out_dir: &mut PathBuf, item: &DocItem) -> Result<()> {
    let (name, content) = match item {
        DocItem::Function {
            name,
            docs,
            args,
            ret,
        } => {
            let mut md = format!("# {}\n\n{}\n\n", name, docs);

            if !args.is_empty() {
                md.push_str("## arguments\n\n");

                for (n, t) in args {
                    md.push_str(&format!("- {}: `{}`\n", n, t));
                }

                md.push('\n');
            }

            if let Some(r) = ret {
                md.push_str("## returns\n\n");
                md.push_str(&format!("- `{}`\n", r));
            }

            (name, md)
        }

        DocItem::Struct {
            name,
            docs,
            fields,
            methods,
        } => {
            let mut md = format!("# {}\n\n{}\n\n## fields\n\n", name, docs);

            for (n, t, d) in fields {
                if d.is_empty() {
                    md.push_str(&format!("- {}: `{}`\n", n, t));
                } else {
                    md.push_str(&format!("- {}: `{}` - {}\n", n, t, d));
                }
            }

            if !methods.is_empty() {
                md.push_str("\n## methods\n\n");

                for m in methods {
                    md.push_str(&format!("### {}\n", m.name));

                    if !m.description.is_empty() {
                        md.push_str(&format!("{}\n\n", m.description));
                    }

                    if !m.args.is_empty() {
                        md.push_str("**arguments**\n");

                        for (n, t) in &m.args {
                            let doc = m
                                .doc_args
                                .iter()
                                .find(|d| d.contains(n))
                                .map(|d| format!(" - {}", d))
                                .unwrap_or_default();

                            md.push_str(&format!("- {}: `{}`{}\n", n, t, doc));
                        }

                        md.push('\n');
                    }

                    if m.ret.is_some() || !m.doc_returns.is_empty() {
                        md.push_str("**returns**\n");

                        if let Some(r) = &m.ret {
                            md.push_str(&format!("- `{}`\n", r));
                        }

                        for r in &m.doc_returns {
                            md.push_str(&format!("- {}\n", r));
                        }

                        md.push('\n');
                    }
                }
            }

            (name, md)
        }

        DocItem::Enum {
            name,
            docs,
            variants,
        } => {
            let mut md = format!("# {}\n\n{}\n\n## variants\n\n", name, docs);

            for v in variants {
                md.push_str(&format!("- `{}`\n", v));
            }

            (name, md)
        }

        DocItem::Impl { .. } => {
            return Ok(());
        }
    };

    let file_path = out_dir.join(format!("{}.md", name));

    fs::write(file_path, content)?;

    write_index(out_dir)?;

    Ok(())
}

fn write_index(out_dir: &mut PathBuf) -> Result<()> {
    let mut entries = vec![];

    for entry in std::fs::read_dir(&out_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map(|e| e == "md").unwrap_or(false) {
            let name = path.file_stem().unwrap().to_string_lossy();

            entries.push(format!("- [{}]({}.md)", name, name));
        }
    }

    let folder_name = out_dir.file_name().unwrap().to_string_lossy().to_string();

    let title = folder_name
        .replace("_", " ")
        .split_whitespace()
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let content = format!("# {}\n\n{}", title, entries.join("\n"));

    std::fs::write(out_dir.join("index.md"), content)?;

    Ok(())
}
