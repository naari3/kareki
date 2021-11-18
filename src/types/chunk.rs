use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;

use crate::packet::client;
use crate::packet::client::ChunkData;
use crate::protocol::ProtocolWrite;
use crate::types::heightmap::Heightmaps;
use crate::types::nbt::Nbt;

use super::chunk_section::ChunkSection;

#[derive(Debug, Clone)]
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

    pub fn to_packet(self, chunk_x: i32, chunk_z: i32) -> Result<client::PlayPacket> {
        let mut data = vec![];
        for section in self.sections.iter() {
            ChunkSection::proto_encode(section, &mut data)?;
        }

        let packet = client::PlayPacket::ChunkData(ChunkData {
            chunk_x,
            chunk_z,
            full_chunk: true,
            primary_bit_mask: 0b1111111111111111.into(),
            heightmaps: Nbt(Heightmaps::from_array(&[0; 256])),
            biomes: Some(vec![0.into(); 1024]),
            data,
            block_entities: vec![],
        });

        Ok(packet)
    }
}
