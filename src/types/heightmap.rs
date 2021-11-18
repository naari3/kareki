use std::{fmt, marker::PhantomData};

use serde::{
    de::{self, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heightmaps {
    #[serde(rename = "MOTION_BLOCKING")]
    #[serde(serialize_with = "nbt::i64_array")]
    #[serde(deserialize_with = "deserialize_i64_36")]
    pub motion_blocking: [i64; 36],
}

// TODO: 絶対もっといい書き方があると思うんだけど
impl Heightmaps {
    pub fn from_array(array: &[u16; 256]) -> Self {
        let mut yoyobits = [0i64; 36];
        let bits_per_byte = 9;
        // let length = (array.len() + values_per_u64 - 1) / values_per_u64;
        let mask = (1 << bits_per_byte) - 1;
        let mut bits = [0u64; 36];

        for (index, &value) in array.iter().enumerate() {
            let all_bit_index = index * bits_per_byte;
            let bit_index = all_bit_index % 64;
            let u64_index = all_bit_index / 64;
            let is_over = (bit_index + bits_per_byte) % 64 < bits_per_byte
                && (bit_index + bits_per_byte) % 64 != 0;
            let u64 = &mut bits[u64_index];
            *u64 &= !(mask << bit_index);
            *u64 |= (value as u64) << bit_index;
            if is_over {
                let u64_index = u64_index + 1;
                let bit_index = (bit_index + bits_per_byte) % 64;
                let next_u64 = &mut bits[u64_index];
                *next_u64 &= !(mask << bit_index);
                *next_u64 |= (value as u64) << bit_index;
            }
        }

        for (index, &value) in bits.iter().enumerate() {
            yoyobits[index] = value as i64;
        }

        Self {
            motion_blocking: yoyobits,
        }
    }
}

fn deserialize_i64_36<'de, D>(deserializer: D) -> Result<[i64; 36], D::Error>
where
    D: Deserializer<'de>,
{
    struct MaxVisitor(PhantomData<fn() -> [i64; 36]>);

    impl<'de> Visitor<'de> for MaxVisitor {
        type Value = [i64; 36];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of 36 numbers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<[i64; 36], S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut res = [0; 36];
            let mut index: usize = 0;

            while let Some(value) = seq.next_element()? {
                res[index] = value;
                index += 1;
            }

            if index != 36 {
                return Err(de::Error::custom(format!(
                    "expected 36 numbers, found {}",
                    index
                )));
            }

            Ok(res)
        }
    }

    // Create the visitor and ask the deserializer to drive it. The
    // deserializer will call visitor.visit_seq() if a seq is present in
    // the input data.
    let visitor = MaxVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}
