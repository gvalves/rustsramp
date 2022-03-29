use std::collections::HashMap;
use std::env;

use once_cell::sync::OnceCell;

use crate::Result;

use self::strategies::{BasicCliBuilderStrategy, BasicExtractorStrategy, BasicValidatorStrategy};

pub mod strategies;

static OPTIONS_MAP: OnceCell<HashMap<&'static str, ArgKind>> = OnceCell::new();

fn get_options_map() -> &'static HashMap<&'static str, ArgKind> {
    OPTIONS_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("--src", ArgKind::Source);
        m.insert("--out-dir", ArgKind::OutDir);
        m.insert("--verbose", ArgKind::Verbose);
        m
    })
}

pub trait ExtractorStrategy {
    fn extract(&self, args: env::Args) -> Vec<CliArg>;
}

pub trait ValidatorStrategy {
    fn validate(&self, args: &Vec<CliArg>) -> Result;
}

pub trait CliBuilderStrategy {
    fn build(&self, dependencies: CliDependencies) -> Result<Cli>;
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
    fn new(
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

    pub fn builder() -> CliBuilder {
        CliBuilder::default()
    }

    pub fn has_arg(&self, kind: ArgKind) -> bool {
        self.args.iter().any(|arg| arg.kind() == &kind)
    }

    pub fn arg(&self, kind: ArgKind) -> &str {
        self.args
            .iter()
            .find(|arg| arg.kind() == &kind)
            .unwrap()
            .value()
    }

    /// Get a reference to the cli's args.
    pub fn args(&self) -> &[CliArg] {
        self.args.as_ref()
    }
}

pub struct CliBuilder {
    build_strategy: Box<dyn CliBuilderStrategy>,
    dependencies: CliDependencies,
}

impl CliBuilder {
    pub fn build(self) -> Result<Cli> {
        self.build_strategy.build(self.dependencies)
    }

    pub fn set_extractor(&mut self, extractor: Box<dyn ExtractorStrategy>) {
        self.dependencies.extractor = extractor;
    }

    pub fn set_validator(&mut self, validator: Box<dyn ValidatorStrategy>) {
        self.dependencies.validator = validator;
    }
}

impl Default for CliBuilder {
    fn default() -> Self {
        Self {
            build_strategy: Box::new(BasicCliBuilderStrategy),
            dependencies: CliDependencies::default(),
        }
    }
}

pub struct CliDependencies {
    pub args: env::Args,
    pub extractor: Box<dyn ExtractorStrategy>,
    pub validator: Box<dyn ValidatorStrategy>,
}

impl Default for CliDependencies {
    fn default() -> Self {
        Self {
            args: env::args(),
            extractor: Box::new(BasicExtractorStrategy),
            validator: Box::new(BasicValidatorStrategy),
        }
    }
}
