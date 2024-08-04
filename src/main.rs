use clap::Parser;
use std::fs;

mod hex;
mod tests;

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

    /// dump hex in plain text format
    #[arg(long, short, default_value_t = false)]
    plain: bool,

    /// Limit of n bytes before stopping
    #[arg(long, short, default_value_t = 0)]
    limit: usize,

    /// Skip to offset n
    #[arg(long, short, default_value_t = 0)]
    skip: usize,

    /// Dump binary instead of hexadecimal. Incompatible with (-p, -i)
    #[arg(long, short, default_value_t = false)]
    binary: bool,

    /// Dump hex in uppercase format
    #[arg(long, short, default_value_t = false)]
    uppercase: bool,

    /// Print offset in decimal. default is Hex
    #[arg(long, short, default_value_t = false)]
    decimal: bool,
}

fn main() {
    let args = Args::parse();
    let file_content = fs::read_to_string(args.file_name.clone());

    if let Err(e) = file_content {
        eprintln!("{}", e.to_string());
        std::process::exit(64);
    }

    let file_content = file_content.unwrap();
    let mut hex = Hex::new(
        file_content, args.characters,
        args.group, args.limit, args.skip,
        args.binary, args.uppercase,
        args.decimal,
    );

    let result = if args.include {
        hex.dump_c_array(args.file_name)
    } else if args.plain {
        hex.dump_plain_hex()
    } else {
        hex.dump_bytes()
    };

    println!("{result}");
}
