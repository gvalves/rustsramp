use std::fs::File;
use std::io::Write;

use crate::domain::entities::{
    drach::{DrachContext, DrachNeighbor, DrachNeighborPosition},
    Drach,
};
use crate::Result;

pub(crate) trait WriteStrategy {
    fn write(&self, file: &File, neighbor: DrachNeighbor) -> Result;
}

pub(crate) struct WriteDrachNeighbor<'a> {
    file: &'a File,
    write_strategy: Box<dyn WriteStrategy>,
}

impl<'a> WriteDrachNeighbor<'a> {
    pub fn new(file: &'a File, write_strategy: Box<dyn WriteStrategy>) -> Self {
        Self {
            file,
            write_strategy,
        }
    }

    pub fn write(
        &mut self,
        drach: &Drach,
        ctx: &DrachContext,
        position: DrachNeighborPosition,
        length: usize,
    ) -> Result {
        let mut builder = DrachNeighbor::builder();
        let neighbor = builder
            .set_drach(drach)
            .set_context(ctx.clone())
            .set_position(position)
            .set_length(length)
            .build()
            .unwrap();

        self.write_strategy.write(self.file, neighbor)
    }
}

pub(crate) struct BasicWriteStrategy;

impl WriteStrategy for BasicWriteStrategy {
    fn write(&self, file: &File, neighbor: DrachNeighbor) -> Result {
        let mut file = file;
        write!(file, "{}", neighbor)?;
        Ok(())
    }
}

pub(crate) struct VerboseWriteStrategy;

impl WriteStrategy for VerboseWriteStrategy {
    fn write(&self, file: &File, neighbor: DrachNeighbor) -> Result {
        let mut file = file;
        let drach = neighbor.drach();
        writeln!(file, "{}", drach.payload())?;
        writeln!(
            file,
            "{} em {}-{}",
            drach.index() + 1,
            drach.start() + 1,
            drach.end()
        )?;
        writeln!(
            file,
            "{}: {}\n",
            match neighbor.position() {
                DrachNeighborPosition::Left => "Anterior",
                DrachNeighborPosition::Right => "Posterior",
            },
            neighbor
        )?;
        Ok(())
    }
}
