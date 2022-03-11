use std::{
    fs::{self},
    io::{self, Write},
};

use crate::{ends_with_any, GenericResult};

pub mod strategies;

pub trait LoadStrategy {
    fn load(&self, path: &str) -> GenericResult<Vec<Sequence>>;
}

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
    pub fn load(path: &str, strategy: Box<dyn LoadStrategy>) -> GenericResult<Vec<Sequence>> {
        strategy.load(path)
    }

    pub fn save(&self, path: &str, append: bool) -> io::Result<()> {
        if !ends_with_any(path, vec![".fasta", ".fas"]) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Filename must be fasta format".to_owned(),
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

#[cfg(test)]
mod tests {
    mod to_fasta {
        use crate::sequence::Sequence;

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
        use std::{
            fs::{self, File},
            path::Path,
        };

        use rand::Rng;

        use crate::{sequence::Sequence, GenericResult};

        #[test]
        fn should_only_save_fasta() {
            let seq = Sequence::new("", "", "", None);
            let res = seq.save("mock.notfasta", false);

            if let Err(err) = res {
                assert_eq!(err.to_string(), "Filename must be fasta format");
            }
        }

        #[test]
        fn should_create_file_if_not_exists() -> GenericResult {
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
        fn should_append_if_append_is_true() -> GenericResult {
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
