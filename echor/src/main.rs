use clap::{Arg, ArgAction, Command};

fn get_args() -> (Vec<String>, bool) {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com")
        .about("Rust version of 'echo'")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();

        (matches.get_many("text").unwrap().cloned().collect(), 
         matches.get_flag("omit_newline"),)
}

fn main() {
    let text: Vec<String>;
    let omit_newline: bool;

    (text, omit_newline) = get_args();

    print!("{}{}", text.join(" "), if omit_newline {""} else {"\n"});
}
