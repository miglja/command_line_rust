use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of 'cat'
struct Args {
    /// Input file(s)
    #[arg(value_name("FILE"), default_value("-"))]
    files: Vec<String>,
    /// equivalent to -vET
    #[arg(short('A'), long("show-all"))]
    show_all: bool,
    /// Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
    /// equivalent to -vE
    #[arg(short('e'))]
    show_nonprint_ends: bool,
    /// display $ at end of each line
    #[arg(short('E'), long("show-ends"))]
    show_ends: bool,
    /// Number lines
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,
    /// suppress repeated empty output lines
    #[arg(short('s'), long("squeeze-blank"))]
    squeeze_blank: bool,
    /// equivalent to -vT
    #[arg(short('t'))]
    show_nonprint_tabs: bool,
    /// display TAB characters as ^I
    #[arg(short('T'), long("show-tabs"))]
    show_tabs: bool,
    /// use ^ and M- notation, except for LFD and TAB
    #[arg(short('v'), long("show-nonprinting"))]
    show_nonprinting: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    // Open stdin or file for reading, depending on command-line input
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn show_nonprinting_chars(line: String) -> String {
    // Replace any non-printing character in the line with its caret-encoded equivalent.

    // \\u{09} - ^I (TAB) and \\u{0A} - ^J (LFD) not provided as the cat usage
    // says -v does not include them
    let caret_encoding: HashMap<&str,&str> = HashMap::from([("\u{00}","^@",),
        ("\\u{01}","^A",), ("\\u{02}","^B",), ("\\u{03}","^C",), ("\\u{04}","^D",), 
        ("\\u{05}","^E",), ("\\u{06}","^F",), ("\\u{07}","^G",), ("\\u{08}","^H",),
        ("\\u{0b}","^K",), ("\\u{0c}","^L",),("\\u{0d}","^M",), ("\\u{0e}","^N",),
        ("\\u{0f}","^O",), ("\\u{10}","^P",), ("\\u{11}","^Q",), ("\\u{12}","^R",),
        ("\\u{13}","^S",), ("\\u{14}","^T",), ("\\u{15}","^U",), ("\\u{16}","^V",),
        ("\\u{17}","^W",), ("\\u{18}","^X",), ("\\u{19}","^Y",), ("\\u{1a}","^Z",),
        ("\\u{1b}","^[",), ("\\u{1c}","^\\",), ("\\u{1d}","^]",), ("\\u{1e}","^^",),
        ("\\u{1f}","^_",), ("\\u{7f}","^?",)]);

    let mut converted = line.clone();
    for c in line.chars() {
        let ce: &str = &format!("{}", c.escape_default());
        if caret_encoding.contains_key(ce) {
            converted = converted.replace(c, caret_encoding[ce]);
        }
    }
    converted
}

fn run(mut args: Args) -> Result<()> {
    // Output file contents based on information received from command-line

    // initialize variable to track whether the last line was blank
    // for multiple blank line suppression.
    // Needs to be out here to correctly mimic original functions behavior
    // if one file ends in multiple blank lines and the next starts with one.
    let mut last_blank = false;

    if args.show_all {
        args.show_nonprinting = true;
        args.show_ends = true;
        args.show_tabs = true;
    }
    if args.show_nonprint_ends {
        args.show_nonprinting = true;
        args.show_ends = true;        
    }
    if args.show_nonprint_tabs {
        args.show_nonprinting = true;
        args.show_tabs = true;       
    }

    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(source) => {
                // initialize variable for line numbering
                let mut count = 1;

                for line in source.lines() {
                    let mut line = if args.show_tabs {line?.replace('\t', "^I")}
                               else {line?};

                    if args.show_nonprinting {
                        line = show_nonprinting_chars(line);
                    }

                    // if blank line suppression set and the line is empty,
                    // skip printing if last line was empty
                    if args.squeeze_blank && line.is_empty() && last_blank {
                        // ... 
                        continue;
                    }

                    // process line numbering if either flag is set.
                    if args.number_lines
                       || (args.number_nonblank_lines && !line.is_empty()) {
                            print!("{count:>6}\t");
                            count += 1;
                    }

                    // print line with or without endline charaacter, depending
                    // on flag.
                    println!("{}{}", line, if args.show_ends {"$"} else {""});
                    // set variable for multiple blank line suppression based
                    // on current line contents.
                    if line.is_empty() {last_blank = true} else {last_blank = false};
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