use clap::Parser;
use std::{fs, io::{self, BufRead, Write}};
use std::path::Path;

mod hex;
mod tests;

use hex::Hex;

const FILE_NAME: &'static str = "";

/// Hexdump a file to stdout. If no file is listed, copy from stdin.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Dump binary instead of hexadecimal. Incompatible with (-p, -i)
    #[arg(long, short, default_value_t = false)]
    binary: bool,

    /// Show n bytes per line.
    #[arg(long, short, default_value_t = 16)]
    characters: u8,

    /// Print offset in decimal. default is Hex
    #[arg(long, short, default_value_t = false)]
    decimal: bool,

    /// File path.
    #[arg(default_value_t = FILE_NAME.to_string())]
    file_name: String,

    /// Group bytes by adding a ' ' every n bytes.
    #[arg(long, short, default_value_t = 2)]
    group: u8,

    /// Output in C array elements
    #[arg(long, short, default_value_t = false)]
    include: bool,

    /// Limit of n bytes before stopping
    #[arg(long, short, default_value_t = 0)]
    limit: usize,

    /// dump hex in plain text format
    #[arg(long, short, default_value_t = false)]
    plain: bool,

    /// Skip to offset n
    #[arg(long, short, default_value_t = 0)]
    skip: usize,

    /// Dump hex in uppercase format
    #[arg(long, short, default_value_t = false)]
    uppercase: bool,
}

fn main() {
    let args = Args::parse();
    let file_content = fs::read_to_string(args.file_name.clone())
        .unwrap_or("".to_string());
    if args.file_name.trim() != "" && !Path::new(&args.file_name).exists() {
        eprintln!("Sorry, provided file does not exist");
        std::process::exit(65);
    }

    let mut hex = Hex::new(
        file_content.clone(), args.characters,
        args.group, args.limit, args.skip,
        args.binary, args.uppercase,
        args.decimal,
    );

    if args.file_name.trim() == "".to_string() {
        run_prompt(hex.clone(), args.plain);
    } else {
        let result = if args.include {
            hex.dump_c_array(args.file_name)
        } else if args.plain {
            hex.dump_plain_hex()
        } else {
            hex.dump_bytes()
        };

        println!("{result}");
    }
}

fn run_prompt(mut hex: Hex, plain: bool) {
    loop {
        let mut input = String::new();
        io::stdout().flush().expect("error flushing");

        let status = io::stdin()
            .lock()
            .read_line(&mut input)
            .expect("Unexpected thing happend");

        if status == 0 { std::process::exit(0); }

        hex.content = input.clone();

        let result = if plain {
            hex.dump_plain_hex()
        } else {
            hex.dump_bytes()
        };

        println!("{result}");
    }
}
