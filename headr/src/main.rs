use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of head
struct Args {
    /// Input file(s)
    #[arg(value_name("FILE"),
          default_value("-")
    )]
    files: Vec<String>,
    /// Number of lines
    #[arg(value_name("LINES"),
          short('n'),
          long,
          default_value("10"),
          value_parser(clap::value_parser!(u64).range(1..))
    )]
    lines: u64,
    /// Number of bytes
    #[arg(value_name("BYTES"),
          short('c'),
          long, conflicts_with("lines"),
          value_parser(clap::value_parser!(u64).range(1..))
    )]
    bytes: Option<u64>,
}


fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    // Open stdin or a file for reading, depending on filename passed.
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


fn run(args: Args) -> Result<()> {
    for (file_count, filename) in args.files.iter().enumerate() {
        match open(&filename) {
            // If there is a problem opening the file, note it and move on.
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut current_file) => {
                if args.files.len() > 1 {
                    // if we have more than one file print the header, preceded
                    // by a newline for every file but the first.
                    println!("{}==> {filename} <==",
                             if file_count > 0 {"\n"} else {""}
                    );
                }
                // READ BYTES
                // if args.bytes has a value and is not None, read up to that
                // value number of bytes and print what we read.
                if let Some(num_bytes) = args.bytes {
                    let mut buf = vec![0; num_bytes as usize];
                    // Might not be enough bytes to read the desired number,
                    // so we determine how many we actually read...
                    let bytes_read = current_file.read(&mut buf)?;
                    // ...and print out that many bytes as a lossy String.
                    print!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
                /*
                if args.bytes.is_some() {
                    let mut buf = vec![0; args.bytes.unwrap() as usize];
                    current_file.read(&mut buf)?;
                    buf.retain(|n| *n != 0);
                    print!("{}", String::from_utf8_lossy(&buf));
                */
                // READ LINES
                // if we're not reading bytes, then we're reading lines.
                } else {
                    // let lines_in_file = args.lines.try_into().unwrap();
                    // let mut line_count = 0;
                    let mut buf = String::new();
                    /*
                    while let Ok(line) = current_file.read_line(&mut buf) {
                        line_count += 1;
                        if (line == 0) | (line_count > lines_in_file) {
                            break;
                        }
                    */
                    // try to read the desired number of lines.
                    for _ in 0..args.lines {
                        // using read_line() instead of lines() to preserve
                        // line endings.
                        let line = current_file.read_line(&mut buf)?;
                        // if we reach the end of the file before reading the
                        // requested number of lines, break out of the loop,
                        // we're done.
                        if line == 0 {
                            break;
                        }
                        print!("{}", buf);
                        buf.clear();
                    }
                }
            },
        }
    }

    Ok(())
}


fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
