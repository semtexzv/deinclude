extern crate clap;

use clap::{App, Arg, Values};
use deinclude::deinclude;
use std::io::Write;

pub fn main() -> std::io::Result<()> {
    let matches = App::new("de-includer")
        .version("0.1.0")
        .about("Resolve includes without running the preprocessor")
        .arg(Arg::with_name("INPUT")
            .help("Input file")
            .required(true)
            .index(1)
        )
        .arg(Arg::with_name("output")
            .help("Output file")
            .long("output")
            .short("o")
            .takes_value(true)
        )
        .arg(Arg::with_name("ignore")
            .help("Ignore files named")
            .short("i")
            .long("ignore")
            .takes_value(true)
            .multiple(true)
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();

    let ignores: Vec<String> = matches.values_of("ignore").unwrap_or(Values::default()).map(|v| v.to_string()).collect();

    let mut ostream: Box<Write> = if let Some(out) = matches.value_of("output") {
        Box::new(std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(out)?)
    } else {
        Box::new(std::io::stdout())
    };

    write!(ostream, "{}", deinclude(input, &ignores)?)?;
    Ok(())
}
