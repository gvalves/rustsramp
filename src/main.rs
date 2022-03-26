use std::process;

use rustsramp::Cli;

fn main() {
    let cli = match Cli::builder().build() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Cli error: {}", err);
            process::exit(1);
        }
    };

    if let Err(err) = rustsramp::run(cli) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
