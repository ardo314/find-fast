use anyhow::{Context, Result};
use glob::glob;
use rayon::prelude::*;
use regex::Regex;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 3 {
        eprintln!("Usage: {} '<glob-pattern>' '<regex>'", args[0]);
        eprintln!("Example: {} '**/*.rs' 'fn.*main'", args[0]);
        eprintln!("\nNote: Quote the glob pattern to prevent shell expansion!");
        std::process::exit(1);
    }

    let glob_pattern = &args[1];
    let regex_pattern = &args[2];

    // Collect all matching files
    let files: Vec<PathBuf> = glob(glob_pattern)
        .with_context(|| format!("Invalid glob pattern: {}", glob_pattern))?
        .filter_map(|entry| entry.ok())
        .filter(|path| path.is_file())
        .collect();

    // Search files for regex pattern    
    let re = Regex::new(regex_pattern)
        .with_context(|| format!("Invalid regex pattern: {}", regex_pattern))?;
    let results: Vec<_> = files
        .par_iter()
        .filter_map(|path| search_file(path, &re))
        .collect();

    // Print results
    for (path, matches) in results {
        println!("{}:", path.display());
        for (line_num, line_content) in matches {
            println!("  {}: {}", line_num, line_content.trim_end());
        }
    }

    Ok(())
}

fn search_file(path: &PathBuf, re: &Regex) -> Option<(PathBuf, Vec<(usize, String)>)> {
    let file = fs::File::open(path).ok()?;
    let reader = BufReader::new(file);

    let matches: Vec<(usize, String)> = reader
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let line = line.ok()?;
            if re.is_match(&line) {
                Some((idx + 1, line)) // Line numbers are 1-indexed
            } else {
                None
            }
        })
        .collect();

    if matches.is_empty() {
        None
    } else {
        Some((path.clone(), matches))
    }
}

