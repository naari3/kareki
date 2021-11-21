use std::{collections::HashMap, io::Result};

use kareki_data::block::Block;

use crate::types::chunk::Chunk;

#[derive(Debug, Clone, Default)]
pub struct World {
    chunks: HashMap<(i32, i32), Chunk>,
}

impl World {
    pub fn new() -> Result<Self> {
        let mut world = Self::default();
        let mut default_chunk = Chunk::empty();
        for x in 0..16 {
            for z in 0..16 {
                for y in 0..16 {
                    let block = if y == 15 {
                        Block::GrassBlock
                    } else {
                        Block::Dirt
                    };
                    default_chunk.set_block(x, y, z, block)?;
                }
            }
        }
        for x in -2..2 {
            for z in -2..2 {
                world.chunks.insert((x, z), default_chunk.clone());
            }
        }

        Ok(world)
    }

    // get chunk or chunk generate and return
    pub fn fetch_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> Result<&mut Chunk> {
        let chunk = self
            .chunks
            .entry((chunk_x, chunk_z))
            .or_insert(Self::generate_chunk(chunk_x, chunk_z)?);

        Ok(chunk)
    }

    fn generate_chunk(_chunk_x: i32, _chunk_z: i32) -> Result<Chunk> {
        let mut chunk = Chunk::empty();
        for x in 0..16 {
            for z in 0..16 {
                for y in 0..16 {
                    if (x + y + z) / 4 % 3 != 0 {
                        let block = Block::from_id((((x + y + z) % 13) + 1) as u32)
                            .expect("Unknown block id");
                        chunk.set_block(x, y, z, block)?;
                    }
                }
            }
        }
        Ok(chunk)
    }

    pub fn get_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> Result<Option<&mut Chunk>> {
        let chunk = self.chunks.get_mut(&(chunk_x, chunk_z));
        Ok(chunk)
    }

    #[allow(dead_code)]
    pub fn get_block(&mut self, x: usize, y: usize, z: usize) -> Result<Option<u16>> {
        let chunk_x = (x >> 4) as i32;
        let chunk_z = (z >> 4) as i32;

        let chunk = self.get_chunk(chunk_x, chunk_z)?;

        match chunk {
            Some(chunk) => Ok(chunk.get_block(x & 0b1111, y, z & 0b1111)),
            None => return Ok(None),
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) -> Result<()> {
        let chunk_x = (x >> 4) as i32;
        let chunk_z = (z >> 4) as i32;

        let chunk = self.get_chunk(chunk_x, chunk_z)?;

        match chunk {
            Some(chunk) => chunk.set_block(x & 0b1111, y, z & 0b1111, block),
            None => return Ok(()),
        }
    }
}
