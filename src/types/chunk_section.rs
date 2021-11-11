use crate::protocol::ProtocolWrite;

use super::{Arr, Var};

#[derive(Debug, Clone)]
pub struct ChunkSection {
    block_count: i16,
    bits_per_block: u8,
    palette: Option<Vec<Var<i32>>>, // Some => indirect, None => direct
    data: Vec<u64>,
}

impl ChunkSection {
    pub fn from_array_and_palette(array: &[u16; 4096], palette: Vec<Var<i32>>) -> Self {
        let mut bits_per_block = (palette.len() as f64).log2().ceil() as u8;
        if bits_per_block < 4 {
            bits_per_block = 4;
        }
        let block_count = array.iter().filter(|&x| *x != 0).count() as i16;
        let values_per_u64 = 64 / bits_per_block as usize;
        let length = (array.len() + values_per_u64 - 1) / values_per_u64;
        let mask = (1 << bits_per_block) - 1;
        let mut bits = vec![0u64; length];

        for (index, &value) in array.iter().enumerate() {
            let u64_index = index / values_per_u64;
            let bit_index = (index % values_per_u64) * (bits_per_block as usize);
            let u64 = &mut bits[u64_index];
            *u64 &= !(mask << bit_index);
            *u64 |= (value as u64) << bit_index;
        }

        Self {
            block_count,
            bits_per_block,
            palette: Some(palette),
            data: bits,
        }
    }
}

impl ProtocolWrite for ChunkSection {
    fn proto_encode(value: &Self, dst: &mut dyn std::io::Write) -> std::io::Result<()> {
        i16::proto_encode(&value.block_count, dst)?;
        u8::proto_encode(&value.bits_per_block, dst)?;
        // TODO: support Option<Arr>
        match &value.palette {
            Some(palette) => <Arr<Var<i32>, Var<i32>>>::proto_encode(palette, dst)?,
            None => {}
        }
        <Arr<Var<i32>, u64>>::proto_encode(&value.data, dst)?;
        Ok(())
    }
}
