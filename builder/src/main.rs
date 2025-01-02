use std::path::PathBuf;
use gen_lib::generate_router_build_script;

pub mod gen_lib;
pub mod rsx;
pub mod transform_book;

fn main() {
    // re-run only if the "example-book" directory changes
    println!("cargo:rerun-if-changed=docs-src/");

    make_docs("articles");
}

fn workspace_root() -> Option<PathBuf> {
    use std::env;
    use std::fs;

    let mut dir = env::current_dir().ok()?;
    loop {
        println!("dir: {}", dir.display());
        let cargo_toml = dir.join("Cargo.toml");
        println!("checking: {}", cargo_toml.display());
        if cargo_toml.exists() {
            if let Ok(content) = fs::read_to_string(&cargo_toml) {
                if content.contains("[workspace]") {
                    return Some(dir);
                }
            }
        }
        if !dir.pop() { break; }
    }
    None
}

fn make_docs(name: &str) {
    let mdbook_dir = workspace_root().unwrap().join("blog");
    println!("mdbook_dir: {}", mdbook_dir.display());
    
    let out_dir = workspace_root().unwrap().join("profile/src/docs");
    println!("out_dir: {}", out_dir.display());
    
    // Ensure the output directory exists
    if !out_dir.exists() {
        std::fs::create_dir_all(&out_dir).expect("Failed to create output directory");
    }

    let mut out = generate_router_build_script(mdbook_dir);
    out.insert_str(0, "use dioxus::prelude::*;\n");
    out.insert_str(0, "use crate::DarkMode;\n");
    out.insert_str(0, "use super::*;\n");

    let filename = format!("router_{}.rs", name.replace(".", ""));

    std::fs::write(out_dir.join(filename), out)
        .expect("Failed to write the generated file to the output directory");
}