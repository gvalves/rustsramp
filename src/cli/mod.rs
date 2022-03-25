use std::collections::HashMap;
use std::env;

use crate::Result;

pub mod strategies;

lazy_static! {
    static ref ARGS_MAP: HashMap<&'static str, ArgKind> = {
        let mut m = HashMap::new();
        m.insert("--src", ArgKind::Source);
        m.insert("--out-dir", ArgKind::OutDir);
        m.insert("--verbose", ArgKind::Verbose);
        m
    };
}

pub trait ExtractorStrategy {
    fn extract(&self, args: env::Args) -> Vec<CliArg>;
}

pub trait ValidatorStrategy {
    fn validate(&self, args: &Vec<CliArg>) -> Result;
}

pub trait CliBuilderStrategy {
    fn build(&self, builder: CliBuilder) -> Result<Cli>;
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum ArgKind {
    Source,
    OutDir,
    Verbose,
}

pub struct CliArg {
    kind: ArgKind,
    value: String,
}

impl CliArg {
    pub fn new(kind: ArgKind, value: &str) -> Self {
        let value = String::from(value);

        Self { kind, value }
    }

    /// Get a reference to the cli arg's kind.
    pub fn kind(&self) -> &ArgKind {
        &self.kind
    }

    /// Get a reference to the cli arg's value.
    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
}

pub struct Cli {
    args: Vec<CliArg>,
}

impl Cli {
    pub fn new(
        args: env::Args,
        extractor: Box<dyn ExtractorStrategy>,
        validator: Box<dyn ValidatorStrategy>,
    ) -> Result<Self> {
        let mut args = args;
        args.next();

        let args = extractor.extract(args);
        validator.validate(&args)?;

        Ok(Self { args })
    }

    pub fn has_arg(&self, kind: ArgKind) -> bool {
        self.args.iter().any(|arg| arg.kind() == &kind)
    }

    /// Get a reference to the cli's args.
    pub fn args(&self) -> &[CliArg] {
        self.args.as_ref()
    }
}

pub struct CliBuilder {
    args: Option<env::Args>,
    extractor: Option<Box<dyn ExtractorStrategy>>,
    validator: Option<Box<dyn ValidatorStrategy>>,
}

impl CliBuilder {
    pub fn new() -> Self {
        Self {
            args: None,
            extractor: None,
            validator: None,
        }
    }

    pub fn build(self, strategy: Box<dyn CliBuilderStrategy>) -> Result<Cli> {
        strategy.build(self)
    }

    /// Get a reference to the cli builder's args.
    pub fn args(&self) -> Option<&env::Args> {
        self.args.as_ref()
    }

    /// Set the cli builder's args.
    pub fn set_args(&mut self, args: Option<env::Args>) {
        self.args = args;
    }

    /// Get a reference to the cli builder's extractor.
    pub fn extractor(&self) -> Option<&Box<dyn ExtractorStrategy>> {
        self.extractor.as_ref()
    }

    /// Set the cli builder's extractor.
    pub fn set_extractor(&mut self, extractor: Option<Box<dyn ExtractorStrategy>>) {
        self.extractor = extractor;
    }

    /// Get a reference to the cli builder's validator.
    pub fn validator(&self) -> Option<&Box<dyn ValidatorStrategy>> {
        self.validator.as_ref()
    }

    /// Set the cli builder's validator.
    pub fn set_validator(&mut self, validator: Option<Box<dyn ValidatorStrategy>>) {
        self.validator = validator;
    }
}
