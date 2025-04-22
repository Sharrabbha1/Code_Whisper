use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use regex::Regex;
use serde::{Serialize};

#[derive(Serialize)]
struct Warning {
    file: String,
    line: usize,
    message: String,
}

fn main() {
    let target_dir = "../test_code";
    let mut warnings = vec![];

    for entry in WalkDir::new(target_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() && entry.path().extension().map(|e| e == "py").unwrap_or(false) {
            let content = fs::read_to_string(entry.path()).unwrap_or_default();
            for (i, line) in content.lines().enumerate() {
                if line.contains("eval(") || line.contains("exec(") {
                    warnings.push(Warning {
                        file: entry.path().display().to_string(),
                        line: i + 1,
                        message: "Use of eval/exec is dangerous".into(),
                    });
                }

                if line.contains("password =") {
                    warnings.push(Warning {
                        file: entry.path().display().to_string(),
                        line: i + 1,
                        message: "Possible hardcoded password".into(),
                    });
                }
            }
        }
    }

    let json = serde_json::to_string_pretty(&warnings).unwrap();
    fs::write("../output/result.json", json).expect("Unable to write file");
    println!("✅ Analysis complete. Check output/result.json");
}
