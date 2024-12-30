use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of wc
struct Args {
    /// Input file(s)
    #[arg(value_name("FILE"), default_value("-"))]
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

#[derive(Default, Debug, PartialEq)]
struct FileInfo {
    line_count: usize,
    word_count: usize,
    byte_count: usize,
    char_count: usize,
 }


fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    // Open stdin or a file for reading, depending on filename passed.
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


fn counter(mut  filename: impl BufRead) -> Result<FileInfo> {
    // read passed file and return a FileInfo struct containing the counts
    // of the various file elements.

    let mut line = String::new();
    let mut file_counts = FileInfo{..Default::default()};

    // using read_line to allow an accurate byte and character
    // count, since it preserves line endings.
    while let Ok(num_bytes) = filename.read_line(&mut line) {
        // break out of loop at end of file
        if num_bytes == 0 { 
            break;
        }

        file_counts.line_count += 1;

        // return value of read_line is number of bytes read, so
        // we can use it as the count here.
        file_counts.byte_count += num_bytes;

        // split_whitespace rather than split ensures all 
        //whitespace is treated as a separator.
        file_counts.word_count += line.split_whitespace().count();

        file_counts.char_count += line.chars().count();

        line.clear();
    }

    Ok(file_counts)
}


fn format_output(count: usize, show: bool) -> std::string::String{
    // format individual file element count for display in report or suppress
    // it, depending on flag.
    if show {
        format!("{:>8}", count)
    } else {
        format!("")
    }
}


fn run(mut args: Args) -> Result<()> {
    // if the user doesn't set any flags, the default is to display 
    // information on lines, words, and bytes.  Set flags accorddingly.
    if [args.lines, args.words, args.bytes, args.chars]
        .iter()
        .all(|v| *v == false)
        {
            args.lines = true;
            args.words = true;
            args.bytes = true;
        }

    let mut totals = FileInfo {..Default::default()};

    for filename in &args.files {
        match open(&filename) {
            // If there is a problem opening the file, note it and move on.
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(current_file) => {
                let current_counts: FileInfo = counter(current_file)?;

                println!("{}{}{}{}{}", 
                    format_output(current_counts.line_count, args.lines),
                    format_output(current_counts.word_count, args.words),
                    format_output(current_counts.byte_count, args.bytes),
                    format_output(current_counts.char_count, args.chars),
                    if filename == "-" {
                        "".to_string()
                    } else {
                        format!(" {filename}")
                    }
                );

                totals.line_count += current_counts.line_count;
                totals.word_count += current_counts.word_count;
                totals.byte_count += current_counts.byte_count;
                totals.char_count += current_counts.char_count;
            },
        };
    }
    if args.files.len() > 1 {
        println!("{}{}{}{} total", 
            format_output(totals.line_count, args.lines),
            format_output(totals.word_count, args.words),
            format_output(totals.byte_count, args.bytes),
            format_output(totals.char_count, args.chars),
        );
    }

    Ok(())
}


fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
