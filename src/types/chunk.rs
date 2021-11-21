use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;

use kareki_data::block::Block;

use crate::packet::client;
use crate::packet::client::ChunkData;
use crate::protocol::ProtocolWrite;
use crate::types::heightmap::Heightmaps;
use crate::types::nbt::Nbt;

use super::chunk_section::ChunkSection;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub sections: Vec<Option<ChunkSection>>,
}

impl Chunk {
    pub fn empty() -> Chunk {
        Self {
            sections: vec![None; 16],
        }
    }

    #[allow(dead_code)]
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<u16> {
        match self.sections.get(y >> 4) {
            Some(section) => match section {
                Some(section) => section.get_block(x, y % 16, z),
                None => None,
            },
            None => None,
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) -> Result<()> {
        self.set_block_raw(x, y, z, block.default_state() as u16)
    }

    pub fn set_block_raw(&mut self, x: usize, y: usize, z: usize, block_id: u16) -> Result<()> {
        match self.sections.get_mut(y >> 4) {
            Some(section) => match section {
                Some(section) => section,
                None => self.create_section(y >> 4),
            }
            .set_block(x, y % 16, z, block_id),
            None => Err(Error::new(ErrorKind::InvalidInput, "out of index")),
        }
    }

    pub fn get_heighest_position(&self, x: usize, z: usize) -> Result<u16> {
        let mut heighest = 0;
        for section in self.sections.iter() {
            for y in 0..16 {
                let section_y = match section {
                    Some(s) => match s.get_block(x, y, z) {
                        Some(y) => y,
                        None => 0,
                    },
                    None => 0,
                };
                if heighest < section_y {
                    heighest = section_y;
                }
            }
        }
        Ok(heighest)
    }

    fn create_section(&mut self, index: usize) -> &mut ChunkSection {
        self.sections[index] = Some(ChunkSection::empty());
        let a = self.sections.get_mut(index);
        a.expect("should valid").as_mut().expect("should valid")
    }

    pub fn to_packet(self, chunk_x: i32, chunk_z: i32) -> Result<client::PlayPacket> {
        let mut data = vec![];
        let mut primary_bit_mask = 0;
        for (section_y, section) in self.sections.iter().enumerate() {
            if let Some(section) = section {
                primary_bit_mask |= 1 << section_y;
                ChunkSection::proto_encode(section, &mut data)?;
            }
        }
        let mut height_map = [0; 256];
        for x in 0..16 {
            for z in 0..16 {
                height_map[(x * 16) + z] = self.get_heighest_position(x, z)?;
            }
        }

        let packet = client::PlayPacket::ChunkData(ChunkData {
            chunk_x,
            chunk_z,
            full_chunk: true,
            primary_bit_mask: primary_bit_mask.into(),
            heightmaps: Nbt(Heightmaps::from_array(&height_map)),
            biomes: Some(vec![127.into(); 1024]),
            data,
            block_entities: vec![],
        });

        Ok(packet)
    }
}
