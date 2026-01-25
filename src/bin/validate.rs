#![allow(dead_code)]

use std::fs;
use std::path::Path;
use std::process::ExitCode;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Tanka {
    top_flavor: String,
    qr_link: String,
    art_link: String,
    #[serde(default)]
    bandcamp_embed: Option<String>,
    #[serde(default)]
    bandcamp_embed_isprivate: bool,
    recommended_music_pairing: MusicPairing,
    tanka: TankaVerses,
    tankadesc: String,
    #[serde(default)]
    tastingnotes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MusicPairing {
    track: String,
    artist: String,
    album: String,
    volume_level: String,
}

#[derive(Debug, Deserialize)]
struct TankaVerses {
    #[serde(rename = "1")]
    v1: String,
    #[serde(rename = "2")]
    v2: String,
    #[serde(rename = "3")]
    v3: String,
    #[serde(rename = "4")]
    v4: String,
    #[serde(rename = "5")]
    v5: String,
}

fn main() -> ExitCode {
    let content_dir = Path::new("content");

    if !content_dir.exists() {
        eprintln!("error: content/ directory not found");
        return ExitCode::FAILURE;
    }

    let mut found = 0;
    let mut passed = 0;
    let mut failed = 0;

    let entries = match fs::read_dir(content_dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("error reading content/: {}", e);
            return ExitCode::FAILURE;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();

        // Skip template and non-yaml files
        if name.contains("template") || !name.ends_with(".yml") {
            continue;
        }

        found += 1;

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("FAIL {}: read error: {}", name, e);
                failed += 1;
                continue;
            }
        };

        match serde_yaml::from_str::<Tanka>(&content) {
            Ok(_) => {
                println!("OK   {}", name);
                passed += 1;
            }
            Err(e) => {
                eprintln!("FAIL {}: {}", name, e);
                failed += 1;
            }
        }
    }

    println!();
    println!("{} found, {} passed, {} failed", found, passed, failed);

    if failed > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
