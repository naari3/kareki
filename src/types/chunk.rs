use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;

use super::chunk_section::ChunkSection;

pub struct Chunk {
    pub sections: Vec<ChunkSection>,
}

impl Chunk {
    pub fn empty() -> Chunk {
        Self {
            sections: vec![ChunkSection::empty(); 16],
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<u16> {
        match self.sections.get(y >> 4) {
            Some(section) => section.get_block(x, y % 16, z),
            None => None,
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_id: u16) -> Result<()> {
        match self.sections.get_mut(y >> 4) {
            Some(section) => section.set_block(x, y % 16, z, block_id),
            None => Err(Error::new(ErrorKind::InvalidInput, "out of index")),
        }
    }
}
