use std::fs;

use crate::GenericResult;

use super::{LoadStrategy, Sequence};

pub struct BasicLoadStrategy;

impl LoadStrategy for BasicLoadStrategy {
    fn load(&self, path: &str) -> GenericResult<Vec<Sequence>> {
        let fasta = fs::read_to_string(path)?;
        let mut seqs = vec![];
        let mut curr_seq = None;

        for line in fasta.split_terminator('\n') {
            if line.starts_with('>') {
                if let Some(seq) = curr_seq.take() {
                    seqs.push(seq);
                }

                let id = match &line[1..].split_once(' ') {
                    Some((id, _)) => id,
                    None => "",
                };

                curr_seq = Some(Sequence::new(id, line, "", None));

                continue;
            }

            if let Some(ref mut seq) = curr_seq {
                seq.payload_mut().push_str(line);
            }
        }

        if let Some(seq) = curr_seq.take() {
            seqs.push(seq);
        }

        Ok(seqs)
    }
}
