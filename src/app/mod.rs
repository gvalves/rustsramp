use std::fs::{self, OpenOptions};

use crate::cli::{ArgKind, Cli};
use crate::domain::entities::{
    drach::{DrachContext, DrachNeighborPosition},
    Drach, Sequence,
};
use crate::domain::usecases::write_drach_neighbor::{
    BasicWriteStrategy, VerboseWriteStrategy, WriteDrachNeighbor, WriteStrategy,
};
use crate::Result;

pub fn run(cli: Cli) -> Result {
    let src_path = cli.arg(ArgKind::Source);
    let out_dir_path = cli.arg(ArgKind::OutDir);
    let is_verbose = cli.has_arg(ArgKind::Verbose);

    let seqs = Sequence::load(src_path)?;

    fs::create_dir_all(out_dir_path)?;

    for seq in seqs {
        let out_path = format!("{}/{}.fasta", out_dir_path, seq.id());
        fs::remove_file(&out_path)?;

        let file = OpenOptions::new()
            .create_new(true)
            .append(true)
            .open(out_path)?;

        let drachs = Drach::from_sequence(&seq);

        let write_strategy: Box<dyn WriteStrategy> = if let true = is_verbose {
            Box::new(VerboseWriteStrategy)
        } else {
            Box::new(BasicWriteStrategy)
        };

        let mut write_drach_neighbor = WriteDrachNeighbor::new(&file, write_strategy);

        let ctx = DrachContext::new(&seq, &drachs);

        for drach in drachs.iter() {
            write_drach_neighbor.write(drach, &ctx, DrachNeighborPosition::Left, 15)?;
            write_drach_neighbor.write(drach, &ctx, DrachNeighborPosition::Right, 15)?;
        }
    }

    Ok(())
}
