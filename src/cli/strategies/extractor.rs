use std::env;

use crate::cli::{get_options_map, CliArg, ExtractorStrategy};

pub struct BasicExtractorStrategy;

impl ExtractorStrategy for BasicExtractorStrategy {
    fn extract(&self, args: env::Args) -> Vec<CliArg> {
        let args: Vec<String> = args.map(|arg| arg).collect();
        let mut args_pos = vec![];
        let mut mapped_args = vec![];
        let options_map = get_options_map();

        for (pos, arg) in args.iter().enumerate() {
            if options_map.iter().any(|(key, _)| key == &arg) && !mapped_args.contains(&arg) {
                args_pos.push(pos);
                mapped_args.push(arg);
            }
        }

        let args: Vec<CliArg> = args_pos
            .into_iter()
            .map(|pos| {
                let (_, &kind) = options_map
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
