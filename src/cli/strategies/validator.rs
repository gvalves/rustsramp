use crate::cli::{ArgKind, CliArg, ValidatorStrategy};
use crate::{Error, Result};

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
