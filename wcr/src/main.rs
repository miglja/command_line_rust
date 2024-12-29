use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of wc
struct Args {
    /// Input file(s)
    #[arg(value_name("FILE"),
          default_value("-")
    )]
    files: Vec<String>,
    /// Show line count
    #[arg(short('l'), long,)]
    lines: bool,
    /// Show word count
    #[arg(short('w'), long,)]
    words: bool,
    /// Show byte count  
    #[arg(short('c'), long, conflicts_with("chars"))]
    bytes:  bool,
    /// Show character count
    #[arg(short('m'), long,)]
    chars: bool,
}


fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    // Open stdin or a file for reading, depending on filename passed.
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


fn run(mut args: Args) -> Result<()> {
    if [args.lines, args.words, args.bytes, args.chars]
        .iter()
        .all(|v| *v == false)
        {
            args.lines = true;
            args.words = true;
            args.bytes = true;
        }

    let mut line_total = 0;
    let mut word_total = 0;
    let mut byte_total = 0;
    let mut char_total = 0;
    for filename in &args.files {
        let mut line_count = 0;
        let mut word_count = 0;
        let mut byte_count = 0;
        let mut char_count = 0;
        match open(&filename) {
            // If there is a problem opening the file, note it and move on.
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut current_file) => {
                let mut line = String::new();
                while let Ok(n) = current_file.read_line(&mut line) {
                    if n == 0 { break; }
                    line_count += 1;
                    line_total += 1;
                    if args.bytes {
                        byte_count += line.len();
                        byte_total += line.len();
                    }
                    if args.words {
                        let words = line.split_whitespace().collect::<Vec<&str>>().len();
                        word_count += words;
                        word_total += words;
                    }
                    if args.chars {
                        let chars = line.chars().collect::<Vec<_>>().len();
                        char_count += chars;
                        char_total += chars;
                    }

                    line.clear();
                }
                if args.lines {
                    print!("{:>8}", line_count);
                }
                if args.words {
                    print!("{:>8}", word_count);
                }
                if args.bytes {
                    print!("{:>8}", byte_count);
                }
                if args.chars {
                    print!("{:>8}", char_count);
                }
                if filename != "-" {
                    println!(" {}", filename);
                } else {
                    println!();
                }
            },
        };
    }
    if args.files.len() > 1 {
        if args.lines {
            print!("{:>8}", line_total);
        }
        if args.words {
            print!("{:>8}", word_total);
        }
        if args.bytes {
            print!("{:>8}", byte_total);
        }
        if args.chars {
            print!("{:>8}", char_total);
        }
        println!(" total");
    }

    Ok(())
}


fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
