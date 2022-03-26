use crate::cli::{Cli, CliBuilderStrategy, CliDependencies};
use crate::Result;

pub struct BasicCliBuilderStrategy;

impl CliBuilderStrategy for BasicCliBuilderStrategy {
    fn build(&self, dependencies: CliDependencies) -> Result<Cli> {
        let CliDependencies {
            args,
            extractor,
            validator,
        } = dependencies;

        Cli::new(args, extractor, validator)
    }
}
