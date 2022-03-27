use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::cli::{ArgKind, Cli};
use crate::domain::entities::drach::{DrachContext, DrachNeighbor, DrachNeighborPosition};
use crate::domain::entities::{Drach, Sequence};
use crate::Result;

pub fn run(cli: Cli) -> Result {
    let src_path = cli
        .args()
        .iter()
        .find(|arg| arg.kind() == &ArgKind::Source)
        .unwrap()
        .value();

    let seqs = Sequence::load(src_path)?;

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

        let drachs = Drach::from_sequence(&seq);

        for (drach_idx, drach) in drachs.iter().enumerate() {
            let context = DrachContext::new(&seq, &drachs);

            let mut builder = DrachNeighbor::builder();
            let l_neighbor = builder
                .set_drach(drach)
                .set_context(context.clone())
                .set_position(DrachNeighborPosition::Left)
                .set_length(15)
                .build()
                .unwrap();

            let mut builder = DrachNeighbor::builder();
            let r_neighbor = builder
                .set_drach(drach)
                .set_context(context)
                .set_position(DrachNeighborPosition::Right)
                .set_length(15)
                .build()
                .unwrap();

            let mut lines = vec![];

            if is_verbose {
                lines.push(format!("{}\n", drach.payload()));
                lines.push(format!(
                    "{} em {}-{}\n",
                    drach_idx + 1,
                    drach.start() + 1,
                    drach.end()
                ));
                lines.push(format!("Anterior: {}\n", l_neighbor));
                lines.push(format!("Posterior: {}\n\n", r_neighbor));
            } else {
                lines.push(l_neighbor.to_string());
                lines.push(r_neighbor.to_string());
            }

            for line in lines {
                write!(file, "{}", line)?;
            }
        }
    }

    Ok(())
}
