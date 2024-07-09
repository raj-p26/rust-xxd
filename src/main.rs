use clap::Parser;
use std::fs;

mod hex;

use hex::Hex;

/// Hexdump a file to stdout. If no file is listed, copy from stdin.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Show n bytes per line.
    #[arg(long, short, default_value_t = 16)]
    characters: u8,
    /// Group bytes by adding a ' ' every n bytes.
    #[arg(long, short, default_value_t = 2)]
    group: u8,
    /// File path.
    file_name: String,

    /// Output in C array elements
    #[arg(long, short, default_value_t = false)]
    include: bool,

}

fn main() {
    let args = Args::parse();
    let file_content = fs::read_to_string(args.file_name.clone());
    let file_name = args.file_name.replace(".", "_").to_lowercase();

    if let Err(e) = file_content {
        eprintln!("{}", e.to_string());
        std::process::exit(64);
    }

    let file_content = file_content.unwrap();
    let hex = Hex::new(file_content, args.characters, args.group);

    let result = if args.include {
        hex.dump_c_array(file_name)
    } else {
        hex.dump_bytes()
    };
    println!("{result}");
}
