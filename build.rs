//! Build script that auto-discovers tankas in content/

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("tankas_generated.rs");
    let mut f = File::create(&dest_path).unwrap();

    // Scan content/ for yaml files
    let content_dir = Path::new("content");
    let mut tankas: Vec<String> = Vec::new();

    if content_dir.exists() {
        for entry in fs::read_dir(content_dir).unwrap().flatten() {
            let path = entry.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy();

            // Skip templates and non-yaml files
            if name.contains("template") || !name.ends_with(".yml") {
                continue;
            }

            tankas.push(name.to_string());
        }
    }

    // Sort for consistent ordering
    tankas.sort();

    // Generate the code
    writeln!(f, "/// Auto-generated list of tankas").unwrap();
    writeln!(f, "fn all_tankas() -> Vec<TankaEntry> {{").unwrap();
    writeln!(f, "    let files: Vec<(&str, &str)> = vec![").unwrap();

    for filename in &tankas {
        writeln!(
            f,
            "        (\"{}\", include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/content/{}\"))),",
            filename, filename
        ).unwrap();
    }

    writeln!(f, "    ];").unwrap();
    writeln!(f).unwrap();
    writeln!(f, "    files").unwrap();
    writeln!(f, "        .into_iter()").unwrap();
    writeln!(f, "        .filter_map(|(filename, content)| {{").unwrap();
    writeln!(f, "            let tanka = load_tanka(content)?;").unwrap();
    writeln!(f, "            Some(TankaEntry {{").unwrap();
    writeln!(f, "                slug: slugify(filename),").unwrap();
    writeln!(f, "                filename: filename.to_string(),").unwrap();
    writeln!(f, "                tanka,").unwrap();
    writeln!(f, "            }})").unwrap();
    writeln!(f, "        }})").unwrap();
    writeln!(f, "        .collect()").unwrap();
    writeln!(f, "}}").unwrap();

    // Tell Cargo to rerun if content/ changes
    println!("cargo:rerun-if-changed=content/");
}
