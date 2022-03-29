use std::fmt::Display;

use regex::Regex;

use crate::{Error, Result};

use super::Sequence;

pub const DRACH_RE: &'static str = r"([AGU][AG]AC[ACU])";

pub struct Drach {
    payload: String,
    position: DrachPosition,
}

impl Drach {
    #[must_use]
    pub fn new(payload: String, position: DrachPosition) -> Self {
        Self { payload, position }
    }

    pub fn from_sequence(sequence: &Sequence) -> Vec<Drach> {
        let re = Regex::new(DRACH_RE).unwrap();
        let text = sequence.payload();
        let mut drachs = vec![];

        for (index, a_match) in re.find_iter(text).enumerate() {
            let payload = String::from(&text[a_match.range()]);
            let position = DrachPosition::new(index, a_match.start(), a_match.end());
            drachs.push(Drach::new(payload, position));
        }

        drachs
    }

    pub fn index(&self) -> usize {
        self.position.index()
    }

    pub fn start(&self) -> usize {
        self.position.start()
    }

    pub fn end(&self) -> usize {
        self.position.end()
    }

    /// Get a reference to the drach's payload.
    #[must_use]
    pub fn payload(&self) -> &str {
        self.payload.as_ref()
    }
}

pub struct DrachPosition {
    index: usize,
    start: usize,
    end: usize,
}

impl DrachPosition {
    #[must_use]
    pub fn new(index: usize, start: usize, end: usize) -> Self {
        let start = start.clamp(0, end);
        let end = end.clamp(start, end);
        Self { index, start, end }
    }

    /// Get the drach position's index.
    #[must_use]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Get the drach position's start.
    #[must_use]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Get the drach position's end.
    #[must_use]
    pub fn end(&self) -> usize {
        self.end
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum DrachNeighborPosition {
    Left,
    Right,
}

#[derive(Clone)]
pub struct DrachNeighbor<'a> {
    drach: &'a Drach,
    context: DrachContext<'a>,
    position: DrachNeighborPosition,
    length: usize,
}

impl<'a> DrachNeighbor<'a> {
    #[must_use]
    pub fn new(
        drach: &'a Drach,
        context: DrachContext<'a>,
        position: DrachNeighborPosition,
        length: usize,
    ) -> Self {
        Self {
            drach,
            context,
            position,
            length,
        }
    }

    pub fn builder() -> DrachNeighborBuilder<'a> {
        DrachNeighborBuilder::default()
    }

    /// Get the drach neighbor's drach.
    #[must_use]
    pub fn drach(&self) -> &Drach {
        self.drach
    }

    /// Get a reference to the drach neighbor's context.
    #[must_use]
    pub fn context(&self) -> &DrachContext<'a> {
        &self.context
    }

    /// Get the drach neighbor's position.
    #[must_use]
    pub fn position(&self) -> DrachNeighborPosition {
        self.position
    }

    /// Get the drach single neighbor's length.
    #[must_use]
    pub fn length(&self) -> usize {
        self.length
    }
}

impl Display for DrachNeighbor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            drach,
            context,
            position,
            length,
        } = self;

        let start = match position {
            DrachNeighborPosition::Left => drach.start() - length,
            DrachNeighborPosition::Right => drach.end(),
        };

        let end = match position {
            DrachNeighborPosition::Left => drach.start(),
            DrachNeighborPosition::Right => drach.end() + length,
        };

        let overlaping_drachs: Vec<&Drach> = match position {
            DrachNeighborPosition::Left => context
                .drachs
                .iter()
                .filter(|d| d.end() > start && d.end() < drach.start())
                .collect(),
            DrachNeighborPosition::Right => context
                .drachs
                .iter()
                .filter(|d| d.start() < end && d.start() > drach.end())
                .collect(),
        };

        let mut sequence = context.sequence.clone();

        for drach in overlaping_drachs {
            let range = drach.start() - 5..drach.end() + 5;
            sequence.remove_drachs_from_range_mut(range);
        }

        write!(
            f,
            "{}",
            &sequence.payload()[sequence.clamp_range(start..end)]
        )
    }
}

#[derive(Clone)]
pub struct DrachNeighborBuilder<'a> {
    drach: Option<&'a Drach>,
    context: Option<DrachContext<'a>>,
    position: Option<DrachNeighborPosition>,
    length: Option<usize>,
}

impl<'a> DrachNeighborBuilder<'a> {
    pub fn build(&mut self) -> Result<DrachNeighbor, Error> {
        let drach = match self.drach.take() {
            Some(v) => v,
            None => return Err(Error::new("drach must be setted")),
        };

        let context = match self.context.take() {
            Some(v) => v,
            None => return Err(Error::new("context must be setted")),
        };

        let position = match self.position.take() {
            Some(v) => v,
            None => return Err(Error::new("position must be setted")),
        };

        let length = match self.length.take() {
            Some(v) => v,
            None => return Err(Error::new("length must be setted")),
        };

        Ok(DrachNeighbor {
            drach,
            context,
            position,
            length,
        })
    }

    /// Set the drach neighbor builder's drach.
    pub fn set_drach(&mut self, drach: &'a Drach) -> &mut Self {
        self.drach = Some(drach);
        self
    }

    /// Set the drach neighbor builder's context.
    pub fn set_context(&mut self, context: DrachContext<'a>) -> &mut Self {
        self.context = Some(context);
        self
    }

    /// Set the drach neighbor builder's position.
    pub fn set_position(&mut self, position: DrachNeighborPosition) -> &mut Self {
        self.position = Some(position);
        self
    }

    /// Set the drach neighbor builder's length.
    pub fn set_length(&mut self, length: usize) -> &mut Self {
        self.length = Some(length);
        self
    }
}

impl Default for DrachNeighborBuilder<'_> {
    fn default() -> Self {
        Self {
            drach: Default::default(),
            context: Default::default(),
            position: Default::default(),
            length: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct DrachContext<'a> {
    sequence: &'a Sequence,
    drachs: &'a Vec<Drach>,
}

impl<'a> DrachContext<'a> {
    #[must_use]
    pub fn new(sequence: &'a Sequence, drachs: &'a Vec<Drach>) -> Self {
        Self { sequence, drachs }
    }

    /// Get the drach context's sequence.
    #[must_use]
    pub fn sequence(&self) -> &Sequence {
        self.sequence
    }

    /// Get the drach context's drachs.
    #[must_use]
    pub fn drachs(&self) -> &[Drach] {
        self.drachs
    }
}

#[cfg(test)]
mod tests {
    mod drach {
        mod from_sequence {
            use crate::domain::entities::{Drach, Sequence};

            #[test]
            fn returns_empty_vector_if_sequence_doesnt_contains_a_drach() {
                let id = "id";
                let header = "header";
                let payload = &"A".repeat(10);

                let seq = Sequence::new(id, header, payload, None);
                let drachs = Drach::from_sequence(&seq);

                assert!(drachs.is_empty());
            }

            #[test]
            fn returns_expected_numbers_of_drachs() {
                let id = "id";
                let header = "header";
                let payload = "AUUAAAGGUUUAUACCUUCCCAGGUAACAAACCAACCAACUUUCGAUCUCUUGUAGAUCUGUUCUCUAAACGAACUUUAAAAUCUGUGUGGCUGUCACUCGGCUGCAUGCUUAGUGCACUCACGCAGUAUAAUUAAUAACUAAUUACUGUCGUUGACAGGACACGAGUAACUCGUCUAUCUUCUGCAGGCUGCUUACGGUUUCGUCCGUGUUGCAG";

                let seq = Sequence::new(id, header, payload, None);
                let drachs = Drach::from_sequence(&seq);

                assert_eq!(drachs.len(), 6);

                let payload = "UUUUGUAUUUCCCUUAAAUUCCAUAAUCAAGACUAUUCAACCAAGGGUUGAAAAGAAAAAGCUUGAUGGCUUUAUGGGUAGAAUUCGAUCUGUCUAUCCAGUUGCGUCACCAAAUGAAUGCAACCAAAUGUGCCUUUCAACUCUCAUGAAGUGUGAUCAUUGUGGUGAAACUUCAUGGCAGACGGGCGAUUUUGUUAAAGCCACUUGCGAAUUUUG";

                let seq = Sequence::new(id, header, payload, None);
                let drachs = Drach::from_sequence(&seq);

                assert_eq!(drachs.len(), 2);
            }

            #[test]
            fn drachs_positions_should_be_correct() {
                let id = "id";
                let header = "header";
                let payload = "UUUUGUAUUUCCCUUAAAUUCCAUAAUCAAGACUAUUCAACCAAGGGUUGAAAAGAAAAAGCUUGAUGGCUUUAUGGGUAGAAUUCGAUCUGUCUAUCCAGUUGCGUCACCAAAUGAAUGCAACCAAAUGUGCCUUUCAACUCUCAUGAAGUGUGAUCAUUGUGGUGAAACUUCAUGGCAGACGGGCGAUUUUGUUAAAGCCACUUGCGAAUUUUG";

                let seq = Sequence::new(id, header, payload, None);
                let drachs = Drach::from_sequence(&seq);

                assert_eq!(drachs[0].start(), 29);
                assert_eq!(drachs[0].end(), 34);
                assert_eq!(drachs[1].start(), 167);
                assert_eq!(drachs[1].end(), 172);
            }
        }
    }
}
