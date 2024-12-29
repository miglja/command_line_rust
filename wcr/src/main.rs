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
                // using read_line to allow an accurate byte and character
                // count, since it preserves line endings.
                while let Ok(num_bytes) = current_file.read_line(&mut line) {
                    // break out of loop at end of file
                    if num_bytes == 0 { 
                        break;
                    }
                    line_count += 1;
                    // return value of read_line is number of bytes read, so
                    // we can use it as the count here.
                    if args.bytes {
                        byte_count += num_bytes;
                    }
                    if args.words {
                        // split_whitespace rather than split ensures all 
                        //whitespace is treated as a separator.
                        let words = line.split_whitespace().count();
                        word_count += words;
                    }
                    if args.chars {
                        let chars = line.chars().count();
                        char_count += chars;
                    }

                    line.clear();
                }
                if args.lines {
                    print!("{:>8}", line_count);
                    line_total += line_count;
                }
                if args.words {
                    print!("{:>8}", word_count);
                    word_total += word_count;
                }
                if args.bytes {
                    print!("{:>8}", byte_count);
                    byte_total += byte_count;
                }
                if args.chars {
                    print!("{:>8}", char_count);
                    char_total += char_count;
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
