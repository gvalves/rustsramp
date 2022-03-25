use std::fs::{self, OpenOptions};
use std::io::Write;

use regex::Regex;

use crate::cli::{ArgKind, Cli};
use crate::sequence::{strategies::BasicLoadStrategy, Sequence};
use crate::Result;

pub fn run(cli: Cli) -> Result {
    let re = Regex::new(r"([AGU][AG]AC[ACU])")?;

    let src_path = cli
        .args()
        .iter()
        .find(|arg| arg.kind() == &ArgKind::Source)
        .unwrap()
        .value();

    let seqs = Sequence::load(src_path, Box::new(BasicLoadStrategy))?;

    let out_dir_path = cli
        .args()
        .iter()
        .find(|arg| arg.kind() == &ArgKind::OutDir)
        .unwrap()
        .value();

    let is_verbose = cli.has_arg(ArgKind::Verbose);

    fs::create_dir_all(out_dir_path)?;

    for seq in seqs {
        let out_path = format!("{}/{}.fasta", out_dir_path, seq.id());
        fs::remove_file(&out_path)?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(out_path)?;

        let payload = seq.payload();
        let mut payload_gap = payload.clone().to_owned();

        for m in re.find_iter(payload) {
            payload_gap.replace_range(m.range(), &"-".repeat(m.range().count()));
        }

        for (i, m) in re.find_iter(payload).enumerate() {
            let start = m.start();
            let end = m.end();

            let mut lines = vec![];

            if is_verbose {
                lines.push(format!("{}\n", m.as_str()));
                lines.push(format!("{} em {}-{}\n", i + 1, start + 1, end));
                lines.push(format!(
                    "Anterior: {}\n",
                    &payload_gap[(start - 15).clamp(0, start)..start]
                ));
                lines.push(format!(
                    "Posterior: {}\n\n",
                    &payload_gap[end..(end + 15).clamp(end, payload_gap.len())]
                ));
            } else {
                lines.push(payload_gap[(start - 15).clamp(0, start)..start].to_owned());
                lines.push(payload_gap[end..(end + 15).clamp(end, payload_gap.len())].to_owned());
            }

            for line in lines {
                write!(file, "{}", line)?;
            }
        }
    }

    Ok(())
}
