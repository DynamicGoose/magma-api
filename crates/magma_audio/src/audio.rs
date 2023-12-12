use std::{fs::File, io::BufReader};

pub struct Audio(pub BufReader<File>);

impl Audio {
    pub fn new(file: File) -> Self {
        Self(BufReader::new(file))
    }
}
