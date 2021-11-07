use std::{fmt, marker::PhantomData};

use serde::{
    de::{self, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heightmaps {
    #[serde(rename = "MOTION_BLOCKING")]
    #[serde(serialize_with = "nbt::i64_array")]
    #[serde(deserialize_with = "deserialize_i64_37")]
    motion_blocking: [i64; 37],
}

// TODO: 絶対もっといい書き方があると思うんだけど
impl Heightmaps {
    pub fn from_array(array: &[u16; 256]) -> Self {
        let mut motion_blocking = [0; 37];
        for (index, chunk) in array.chunks(7).enumerate() {
            let mut bits: u64 = 0;
            for (i, &v) in chunk.iter().enumerate() {
                let v = v & 0b0000000111111111; // 9 bits
                bits += (v as u64) << i * 9;
            }
            bits <<= 1; // 7 * 9 + 1 = 64
            motion_blocking[index] = bits as i64;
        }
        Self {
            motion_blocking: motion_blocking,
        }
    }
}

fn deserialize_i64_37<'de, D>(deserializer: D) -> Result<[i64; 37], D::Error>
where
    D: Deserializer<'de>,
{
    struct MaxVisitor(PhantomData<fn() -> [i64; 37]>);

    impl<'de> Visitor<'de> for MaxVisitor {
        type Value = [i64; 37];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of 37 numbers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<[i64; 37], S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut res = [0; 37];
            let mut index: usize = 0;

            while let Some(value) = seq.next_element()? {
                res[index] = value;
                index += 1;
            }

            if index != 37 {
                return Err(de::Error::custom(format!(
                    "expected 37 numbers, found {}",
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
