use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;

use crate::protocol::ProtocolWrite;

use super::{Arr, Var};

#[derive(Debug, Clone)]
pub struct ChunkSection {
    data: Vec<u16>,
}
// block_count: i16,
// bits_per_block: u8,
// palette: Option<Vec<Var<i32>>>, // Some => indirect, None => direct
// data: Vec<u64>,
impl ChunkSection {
    pub fn empty() -> ChunkSection {
        Self {
            data: vec![0; 4096],
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<u16> {
        let index = ChunkSection::block_index(x, y, z);
        match index {
            Some(i) => Some(self.data[i]),
            None => None,
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_id: u16) -> Result<()> {
        let index = ChunkSection::block_index(x, y, z);
        match index {
            Some(i) => {
                self.data[i] = block_id;
                Ok(())
            }
            None => Err(Error::new(ErrorKind::InvalidInput, "out of index")),
        }
    }

    fn block_index(x: usize, y: usize, z: usize) -> Option<usize> {
        if x >= 16 || y >= 16 || z >= 16 {
            None
        } else {
            Some((y << 8) | (z << 4) | x)
        }
    }

    pub fn make_palette(&self) -> Vec<Var<i32>> {
        let mut palette = self
            .data
            .iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|&n| Var(n as i32))
            .collect::<Vec<_>>();
        palette.sort_unstable();
        palette.insert(0, 0.into());
        palette
    }

    fn from_array_and_palette(
        array: &[u16],
        palette: Vec<Var<i32>>,
    ) -> (i16, u8, Option<Vec<Var<i32>>>, Vec<u64>) {
        assert!(array.len() == 4096);
        let invert_palette: HashMap<u16, usize> = palette
            .iter()
            .enumerate()
            .map(|(i, &n)| (n.0 as u16, i))
            .collect();

        let array2 = array
            .into_iter()
            .map(|n| *invert_palette.get(n).expect("must") as u16)
            .collect::<Vec<_>>();

        let mut bits_per_block = (palette.len() as f64).log2().ceil() as u8;
        if bits_per_block < 4 {
            bits_per_block = 4;
        }
        let block_count = array2.iter().filter(|&x| *x != 0).count() as i16;
        let values_per_u64 = 64 / bits_per_block as usize;
        let length = (array2.len() + values_per_u64 - 1) / values_per_u64;
        let mask = (1 << bits_per_block) - 1;
        let mut bits = vec![0u64; length];

        for (index, &value) in array2.iter().enumerate() {
            let u64_index = index / values_per_u64;
            let bit_index = (index % values_per_u64) * (bits_per_block as usize);
            let u64 = &mut bits[u64_index];
            *u64 &= !(mask << bit_index);
            *u64 |= (value as u64) << bit_index;
        }

        (block_count, bits_per_block, Some(palette), bits)
    }
}

impl ProtocolWrite for ChunkSection {
    fn proto_encode(value: &Self, dst: &mut dyn std::io::Write) -> std::io::Result<()> {
        let palette = value.make_palette();
        let (block_count, bits_per_block, palette, data) =
            Self::from_array_and_palette(&value.data, palette);

        i16::proto_encode(&block_count, dst)?;
        u8::proto_encode(&bits_per_block, dst)?;
        // TODO: support Option<Arr>
        match &palette {
            Some(palette) => <Arr<Var<i32>, Var<i32>>>::proto_encode(palette, dst)?,
            None => {}
        }
        <Arr<Var<i32>, u64>>::proto_encode(&data, dst)?;
        Ok(())
    }
}
