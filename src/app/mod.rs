use std::fs::{self, File, OpenOptions};

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
    let is_verbose = cli.has_arg(ArgKind::Verbose);
    let seqs = load_seqs(&cli)?;

    prepare_outdir(&cli)?;

    for seq in seqs {
        let drachs = Drach::from_sequence(&seq);
        let ctx = DrachContext::new(&seq, &drachs);

        let file = create_fasta_file(&cli, seq.id())?;
        let write_strategy = get_write_strategy(is_verbose);

        let mut write_drach_neighbor = WriteDrachNeighbor::new(&file, write_strategy);

        for drach in drachs.iter() {
            write_drach_neighbor.write(drach, &ctx, DrachNeighborPosition::Left, 15)?;
            write_drach_neighbor.write(drach, &ctx, DrachNeighborPosition::Right, 15)?;
        }
    }

    Ok(())
}

fn load_seqs(cli: &Cli) -> Result<Vec<Sequence>> {
    let path = cli.arg(ArgKind::Source);
    Sequence::load(path)
}

fn prepare_outdir(cli: &Cli) -> Result {
    let path = cli.arg(ArgKind::OutDir);
    fs::create_dir_all(path)?;
    Ok(())
}

fn create_fasta_file(cli: &Cli, filename: &str) -> Result<File> {
    let path = format!("{}/{}.fasta", cli.arg(ArgKind::OutDir), filename);
    fs::remove_file(&path)?;

    let file = OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(path)?;

    Ok(file)
}

fn get_write_strategy(is_verbose: bool) -> Box<dyn WriteStrategy> {
    if let true = is_verbose {
        Box::new(VerboseWriteStrategy)
    } else {
        Box::new(BasicWriteStrategy)
    }
}
