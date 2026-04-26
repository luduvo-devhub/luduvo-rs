use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use syn::{Item, ItemEnum, ItemFn, ItemStruct};

#[derive(Debug)]
pub enum DocItem {
    Function {
        name: String,
        docs: String,
        args: Vec<(String, String)>,
        ret: Option<String>,
    },

    Struct {
        name: String,
        docs: String,
        fields: Vec<(String, String, String)>,
        methods: Vec<MethodDoc>,
    },

    Enum {
        name: String,
        docs: String,
        variants: Vec<String>,
    },

    Impl {
        target: String,
        methods: Vec<MethodDoc>,
    },
}

#[derive(Debug, Default)]
pub struct ParsedDocs {
    pub description: String,
    pub arguments: Vec<String>,
    pub returns: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug)]
pub struct MethodDoc {
    pub name: String,
    pub description: String,
    pub args: Vec<(String, String)>,
    pub ret: Option<String>,
    pub doc_args: Vec<String>,
    pub doc_returns: Vec<String>,
}

pub fn parse_file(path: &Path) -> Result<Vec<DocItem>> {
    let content = fs::read_to_string(path)?;
    let syntax = syn::parse_file(&content)?;

    let mut structs: HashMap<String, DocItem> = HashMap::new();
    let mut others = Vec::new();
    let mut impls = Vec::new();

    for item in syntax.items {
        match item {
            Item::Struct(s) => {
                let s = parse_struct(s);
                if let DocItem::Struct { name, .. } = &s {
                    structs.insert(name.clone(), s);
                }
            }

            Item::Impl(i) => impls.push(parse_impl(i)),
            Item::Fn(f) => others.push(parse_fn(f)),
            Item::Enum(e) => others.push(parse_enum(e)),

            _ => {}
        }
    }

    for imp in impls {
        if let DocItem::Impl { target, methods } = imp {
            if let Some(DocItem::Struct {
                methods: existing_methods,
                ..
            }) = structs.get_mut(&target)
            {
                existing_methods.extend(methods);
            }
        }
    }

    // Collect final items
    let mut result = Vec::new();

    result.extend(structs.into_values());
    result.extend(others);

    Ok(result)
}

fn parse_fn(func: ItemFn) -> DocItem {
    let name = func.sig.ident.to_string();
    let docs = extract_doc(&func.attrs);

    let args = func
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat) = arg {
                let name = if let syn::Pat::Ident(i) = &*pat.pat {
                    i.ident.to_string()
                } else {
                    "unknown".to_string()
                };

                let ty = quote::quote!(#pat.ty).to_string();
                Some((name, ty))
            } else {
                None
            }
        })
        .collect();

    let ret = match func.sig.output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, ty) => Some(quote::quote!(#ty).to_string()),
    };

    DocItem::Function {
        name,
        docs,
        args,
        ret,
    }
}

fn parse_struct(s: ItemStruct) -> DocItem {
    let name = s.ident.to_string();
    let docs = extract_doc(&s.attrs);

    let mut fields = Vec::new();

    if let syn::Fields::Named(named) = s.fields {
        for field in named.named {
            let name = field.ident.unwrap().to_string();
            let ty = type_to_string(&field.ty);
            let docs = extract_doc(&field.attrs);

            fields.push((name, ty, docs));
        }
    }

    DocItem::Struct {
        name,
        docs,
        fields,
        methods: Vec::new(),
    }
}

fn type_to_string(ty: &syn::Type) -> String {
    quote::quote!(#ty)
        .to_string()
        // generics
        .replace(" <", "<")
        .replace("< ", "<")
        .replace(" >", ">")
        .replace("> ", ">")
        // commas
        .replace(" ,", ",")
        // references
        .replace("& ", "&")
        .replace("&mut ", "&mut ")
}

fn parse_enum(e: ItemEnum) -> DocItem {
    let name = e.ident.to_string();
    let docs = extract_doc(&e.attrs);

    let variants = e.variants.iter().map(|v| v.ident.to_string()).collect();

    DocItem::Enum {
        name,
        docs,
        variants,
    }
}

fn extract_doc(attrs: &[syn::Attribute]) -> String {
    let lines: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(meta) = &attr.meta {
                    if let syn::Expr::Lit(expr) = &meta.value {
                        if let syn::Lit::Str(lit) = &expr.lit {
                            return Some(lit.value());
                        }
                    }
                }
            }
            None
        })
        .collect();

    normalize_doc_lines(lines)
}

fn extract_docs_structured(attrs: &[syn::Attribute]) -> ParsedDocs {
    let lines: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(meta) = &attr.meta {
                    if let syn::Expr::Lit(expr) = &meta.value {
                        if let syn::Lit::Str(lit) = &expr.lit {
                            return Some(lit.value());
                        }
                    }
                }
            }
            None
        })
        .collect();

    parse_doc_sections(lines)
}

fn parse_doc_sections(lines: Vec<String>) -> ParsedDocs {
    let mut desc = Vec::new();
    let mut args = Vec::new();
    let mut returns = Vec::new();
    let mut notes = Vec::new();

    enum Section {
        Description,
        Arguments,
        Returns,
        Notes,
    }

    let mut current = Section::Description;

    for line in lines {
        let l = line.trim();

        if l.starts_with("# arguments") {
            current = Section::Arguments;

            continue;
        }

        if l.starts_with("# returns") {
            current = Section::Returns;

            continue;
        }

        if l.starts_with("# notes") {
            current = Section::Notes;

            continue;
        }

        if l.is_empty() {
            continue;
        }

        match current {
            Section::Description => desc.push(l.to_string()),
            Section::Arguments => args.push(clean_bullet(l)),
            Section::Returns => returns.push(clean_bullet(l)),
            Section::Notes => notes.push(clean_bullet(l)),
        }
    }

    ParsedDocs {
        description: desc.join("\n\n"),
        arguments: args,
        returns,
        notes,
    }
}

fn clean_bullet(line: &str) -> String {
    let cleaned = line.trim_start_matches('*').trim_start_matches('-').trim();

    if let Some(rest) = cleaned.strip_prefix('`') {
        if let Some(end) = rest.find('`') {
            let after = &rest[end + 1..].trim_start();

            if let Some(stripped) = after.strip_prefix('-') {
                return stripped.trim().to_string();
            }

            return after.to_string();
        }
    }

    cleaned.to_string()
}

fn normalize_doc_lines(lines: Vec<String>) -> String {
    lines
        .into_iter()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn impl_target_name(ty: &syn::Type) -> String {
    if let syn::Type::Path(p) = ty {
        p.path.segments.last().unwrap().ident.to_string()
    } else {
        "Unknown".to_string()
    }
}

fn parse_impl(item: syn::ItemImpl) -> DocItem {
    let target = impl_target_name(&item.self_ty);

    let mut methods = Vec::new();

    for i in item.items {
        if let syn::ImplItem::Fn(func) = i {
            let name = func.sig.ident.to_string();
            let parsed = extract_docs_structured(&func.attrs);

            let args = func
                .sig
                .inputs
                .iter()
                .filter_map(|arg| {
                    if let syn::FnArg::Typed(pat) = arg {
                        let name = if let syn::Pat::Ident(i) = &*pat.pat {
                            i.ident.to_string()
                        } else {
                            "unknown".to_string()
                        };

                        let ty = type_to_string(&pat.ty);
                        Some((name, ty))
                    } else {
                        None
                    }
                })
                .collect();

            let ret = match func.sig.output {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_, ty) => Some(type_to_string(&ty)),
            };

            methods.push(MethodDoc {
                name,
                description: parsed.description,
                args,
                ret,
                doc_args: parsed.arguments,
                doc_returns: parsed.returns,
            });
        }
    }

    DocItem::Impl { target, methods }
}
