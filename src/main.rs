use clap::Parser;
use std::fs;

mod hex;

use hex::Hex;

/// Hexdump a file to stdout. If no file is listed, copy from stdin.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Numbers of characters.
    #[arg(long, short, default_value_t = 16)]
    characters: u8,
    /// File path.
    file_name: String,
}

fn main() {
    let args = Args::parse();
    let file_content = fs::read_to_string(args.file_name);

    if let Err(e) = file_content {
        eprintln!("{}", e.to_string());
        std::process::exit(64);
    }

    let file_content = file_content.unwrap();
    let hex = Hex::new(file_content, args.characters);

    hex.dump_bytes();
}
