use std::process;
use clap::{App, Arg};

use minigrep::Config;

fn main() {
    let matches = App::new("Minigrep")
        .arg(Arg::with_name("query").index(1).required(true))
        .arg(Arg::with_name("file").index(2).required(true))
        .arg(
            Arg::with_name("case-insensitive")
                .help("enable case insensitive search")
                .short("i")
                .long("case-insensitive")
        )
        .get_matches();
    let query = matches.value_of("query").unwrap();
    let file = matches.value_of("file").unwrap();
    let args = [format!("minigrep"), query.to_string(), file.to_string()];
    let case_sensitive = !matches.is_present("case-insensitive");

    let config = Config::new(&args, case_sensitive).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error {}", e);

        process::exit(1);
    }
}
