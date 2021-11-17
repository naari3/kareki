use std::{collections::HashMap, io::Result};

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
                    if (x + y + z) / 4 % 3 != 0 {
                        default_chunk.set_block(x, y, z, (((x + y + z) % 8) + 1) as u16)?;
                    }
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
    pub fn fetch_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> Result<&mut Chunk> {
        let chunk = self
            .chunks
            .entry((chunk_x, chunk_z))
            .or_insert(Chunk::empty());
        Ok(chunk)
    }
}
