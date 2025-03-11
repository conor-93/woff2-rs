use std::{fs, io, path::PathBuf};
use std::io::Cursor;
use std::path::Path;
use std::time::Instant;
use clap::Parser;
use thiserror::Error;
use walkdir::WalkDir;
use woff2::convert_woff2_to_ttf;
use woff2::decode::DecodeError;

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Woff(#[from] DecodeError),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Debug, Parser)]
struct Args {
    in_path: PathBuf,
    out_path: PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let input = std::fs::read(args.in_path)?;
    let ttf = convert_woff2_to_ttf(&mut io::Cursor::new(input))?;
    std::fs::write(args.out_path, ttf)?;
    Ok(())
}

#[test]
fn benchmark() {
    let test_assets_dir = Path::new("test_assets");

    // Find (recursively) all woff2 font files
    let woff_files: Vec<_> = WalkDir::new(test_assets_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map(|ext| ext == "woff2").unwrap_or(false))
        .map(|entry| entry.path().to_path_buf())
        .collect();

    if woff_files.is_empty() {
        println!("No WOFF2 files found in {:?}", test_assets_dir);
        return;
    }

    // Warmup
    for woff_path in woff_files.clone() {
        let _relative_path = woff_path.strip_prefix(test_assets_dir).unwrap_or(&woff_path);
        let input = fs::read(&woff_path).expect("Failed to read input file");
        let _input_size = input.len();
        let mut cursor_i = Cursor::new(&input);

        let _start = Instant::now();
        let _ttf = convert_woff2_to_ttf(&mut cursor_i).expect("WOFF2 conversion failed");
    }

    // Benchmark
    for woff_path in woff_files {
        let relative_path = woff_path.strip_prefix(test_assets_dir).unwrap_or(&woff_path);
        let input = fs::read(&woff_path).expect("Failed to read input file");
        let input_size = input.len();
        let mut cursor_i = Cursor::new(&input);

        let start = Instant::now();
        let ttf = convert_woff2_to_ttf(&mut cursor_i).expect("WOFF2 conversion failed");
        let duration = start.elapsed();

        let output_path = woff_path.with_extension("ttf");
        fs::write(&output_path, &ttf).expect("Failed to write output file");

        let output = fs::read(&output_path).expect("Failed to read output file");
        let output_size = output.len();

        println!(
            "{} -> {} converted in {:.2?} - File {}/{}",
            format_bytes_as_kb(input_size),
            format_bytes_as_kb(output_size),
            duration,
            test_assets_dir.display(),
            relative_path.display(),
        );
    }
}

fn format_bytes_as_kb(bytes: usize) -> String {
    format!("{:.2} KB", bytes as f64 / 1024.0)
}

