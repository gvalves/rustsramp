use std::process;

use rustsramp::cli::strategies::cli_builder::BasicCliBuilderStrategy;
use rustsramp::cli::CliBuilder;

fn main() {
    let cli = match CliBuilder::new().build(Box::new(BasicCliBuilderStrategy)) {
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
