use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufReader, BufRead};
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Root directory of your Xcode or project folder
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();

    println!("Scanning project at: {}", args.path);

    for entry in WalkDir::new(&args.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().is_file() &&
            (e.path().extension().map(|ext| ext == "swift" || ext == "h" || ext == "m").unwrap_or(false))
        })
    {
        let path = entry.path();
        println!("📄 {}", path.display());

        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            for (idx, line) in reader.lines().enumerate() {
                if let Ok(content) = line {
                    println!("    {}", content);
                }
                if idx >= 9 { break; } // Only first 10 lines
            }
        }

        println!("---");
    }
}