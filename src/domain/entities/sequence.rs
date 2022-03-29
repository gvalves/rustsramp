use std::fs::{self};
use std::io::{self, Write};
use std::ops::Range;

use rand::Rng;
use regex::Regex;

use crate::utils::ends_with_any;
use crate::Result;

use super::drach::DRACH_RE;

pub const ACCEPTED_FASTA_EXT: [&'static str; 2] = [".fasta", ".fas"];
pub const BASES: [char; 4] = ['A', 'U', 'G', 'C'];

#[derive(Clone)]
pub struct Sequence {
    id: String,
    header: String,
    payload: String,
    origin: Option<Box<Sequence>>,
}

impl Sequence {
    pub fn new(id: &str, header: &str, payload: &str, origin: Option<Box<Sequence>>) -> Self {
        let id = String::from(id);
        let header = String::from(header);
        let payload = String::from(payload);

        Self {
            id,
            header,
            payload,
            origin,
        }
    }

    /// Get a reference to the sequence's id.
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    /// Get a reference to the sequence's header.
    pub fn header(&self) -> &str {
        self.header.as_ref()
    }

    /// Get a reference to the sequence's payload.
    pub fn payload(&self) -> &str {
        self.payload.as_ref()
    }

    /// Get a mutable reference to the sequence's payload.
    fn payload_mut(&mut self) -> &mut String {
        &mut self.payload
    }

    /// Get a reference to the sequence's origin.
    pub fn origin(&self) -> Option<&Box<Sequence>> {
        self.origin.as_ref()
    }
}

impl Sequence {
    pub fn load(path: &str) -> Result<Vec<Sequence>> {
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
                seq.payload_mut().push_str(line.trim());
            }
        }

        if let Some(seq) = curr_seq.take() {
            seqs.push(seq);
        }

        Ok(seqs)
    }

    pub fn save(&self, path: &str, append: bool) -> io::Result<()> {
        if !ends_with_any(path, ACCEPTED_FASTA_EXT.to_vec()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Filename must ends with {:?}", ACCEPTED_FASTA_EXT),
            ));
        }

        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(append)
            .open(path)?;

        writeln!(file, "{}", self.to_fasta(80))?;

        Ok(())
    }

    pub fn to_fasta(&self, line_len: usize) -> String {
        let mut fasta = format!(">{}", self.header());
        let payload = self.payload();
        let payload_len = payload.len();

        for i in (0..payload_len).step_by(line_len) {
            fasta.push_str(&format!(
                "\n{}",
                &payload[i..(i + line_len).clamp(i, payload_len)]
            ))
        }

        fasta
    }
}

impl Sequence {
    pub fn clamp_range(&self, range: Range<usize>) -> Range<usize> {
        let start = range.start.clamp(0, self.payload.len());
        let end = range.end.clamp(0, self.payload.len());
        start..end
    }

    pub fn remove_drachs_from_range(&self, range: Range<usize>) -> String {
        let re = Regex::new(DRACH_RE).unwrap();
        let range = self.clamp_range(range);
        let mut seq_slice: Vec<char> = self.payload[range].chars().map(|c| c).collect();

        loop {
            for i in 0..5 {
                let base = rand::thread_rng().gen_range(0..=3);
                seq_slice[i + 5] = BASES[base];
            }

            let bytes: Vec<u8> = seq_slice.iter().map(|c| *c as u8).collect();
            let new_seq_slice = String::from_utf8(bytes).unwrap();

            if !re.is_match(&new_seq_slice[4..9]) {
                break new_seq_slice;
            }

            seq_slice = new_seq_slice.chars().map(|c| c).collect();
        }
    }

    pub fn remove_drachs_from_range_mut(&mut self, range: Range<usize>) {
        let range = self.clamp_range(range);
        let seq_slice = self.remove_drachs_from_range(range.clone());
        self.payload.replace_range(range, &seq_slice);
    }
}

#[cfg(test)]
mod tests {
    mod to_fasta {
        use crate::domain::entities::Sequence;

        #[test]
        fn should_match_format() {
            let seq = Sequence::new("NC_045512", "NC_045512 Sars-Cov-2", "AGTC", None);
            let expect = concat!(">NC_045512 Sars-Cov-2", "\nAGTC");

            assert_eq!(seq.to_fasta(10), expect);
        }

        #[test]
        fn should_match_format_breaking_lines() {
            let seq = Sequence::new("NC_045512", "NC_045512 Sars-Cov-2", "AGTC", None);
            let expect = concat!(">NC_045512 Sars-Cov-2", "\nAG", "\nTC");

            assert_eq!(seq.to_fasta(2), expect);
        }
    }

    mod save {
        use std::fs::{self, File};
        use std::path::Path;

        use rand::Rng;

        use crate::domain::entities::sequence::ACCEPTED_FASTA_EXT;
        use crate::domain::entities::Sequence;
        use crate::Result;

        #[test]
        fn should_only_save_fasta() {
            let seq = Sequence::new("", "", "", None);
            let res = seq.save("mock.notfasta", false);

            if let Err(err) = res {
                assert_eq!(
                    err.to_string(),
                    format!("Filename must ends with {:?}", ACCEPTED_FASTA_EXT)
                );
            }
        }

        #[test]
        fn should_create_file_if_not_exists() -> Result {
            fs::create_dir_all("./tmp")?;

            let mut path;

            loop {
                path = format!("./tmp/{}.fasta", rand::thread_rng().gen::<u32>());

                if !Path::new(&path).exists() {
                    break;
                }
            }

            let seq = Sequence::new("", "", "", None);
            let res = seq.save(&path, false);

            assert!(res.is_ok());

            fs::remove_file(path)?;

            Ok(())
        }

        #[test]
        fn should_append_if_append_is_true() -> Result {
            fs::create_dir_all("./tmp")?;

            let mut path;

            loop {
                path = format!("./tmp/{}.fasta", rand::thread_rng().gen::<u32>());

                if !Path::new(&path).exists() {
                    break;
                }
            }

            File::create(&path)?;

            let fas = fs::read_to_string(&path)?;
            let seq = Sequence::new("", "", "", None);

            let expect = format!("{}{}\n", fas, seq.to_fasta(80));

            seq.save(&path, true)?;

            let fas = fs::read_to_string(&path)?;

            assert_eq!(fas, expect);

            fs::remove_file(path)?;

            Ok(())
        }
    }
}
