pub mod extractor {
    use std::env;

    use crate::cli::{CliArg, ExtractorStrategy, ARGS_MAP};

    pub struct BasicExtractorStrategy;

    impl ExtractorStrategy for BasicExtractorStrategy {
        fn extract(&self, args: env::Args) -> Vec<CliArg> {
            let args: Vec<String> = args.map(|arg| arg).collect();
            let mut args_pos = vec![];
            let mut mapped_args = vec![];

            for (pos, arg) in args.iter().enumerate() {
                if ARGS_MAP.iter().any(|(key, _)| key == &arg) && !mapped_args.contains(&arg) {
                    args_pos.push(pos);
                    mapped_args.push(arg);
                }
            }

            let args: Vec<CliArg> = args_pos
                .into_iter()
                .map(|pos| {
                    let (_, &kind) = ARGS_MAP
                        .iter()
                        .find(|(key, _)| key == &&args.get(pos).unwrap())
                        .unwrap();

                    let value = match args.get(pos + 1) {
                        Some(val) => val,
                        None => "",
                    };

                    CliArg::new(kind, value)
                })
                .collect();

            args
        }
    }
}

pub mod validator {
    use crate::{
        cli::{ArgKind, CliArg, ValidatorStrategy},
        Error, Result,
    };

    pub struct BasicValidatorStrategy;

    impl ValidatorStrategy for BasicValidatorStrategy {
        fn validate(&self, args: &Vec<CliArg>) -> Result {
            if !args.iter().any(|arg| arg.kind() == &ArgKind::Source) {
                return Err(Box::new(Error::new("Missing --src argument")));
            }

            if !args.iter().any(|arg| arg.kind() == &ArgKind::OutDir) {
                return Err(Box::new(Error::new("Missing --out-dir argument")));
            }

            Ok(())
        }
    }
}

pub mod cli_builder {
    use crate::{
        cli::{Cli, CliBuilderStrategy, CliDependencies},
        Result,
    };

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
}
