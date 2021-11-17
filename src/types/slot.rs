use std::io::{Read, Result};

use crate::protocol::ProtocolRead;

use super::{item_stack_meta::ItemStackMeta, nbt::Nbt, Var};

#[derive(Debug, Clone, Default)]
pub struct Slot {
    pub present: bool,
    pub item_id: Option<Var<i32>>,
    pub item_count: Option<u8>,
    pub meta: Option<Nbt<ItemStackMeta>>,
}

impl ProtocolRead for Slot {
    fn proto_decode<S: Read>(src: &mut S) -> Result<Self> {
        let present = bool::proto_decode(src)?;
        let item_id = if present {
            Some(<Var<i32>>::proto_decode(src)?)
        } else {
            None
        };
        let item_count = if present {
            Some(u8::proto_decode(src)?)
        } else {
            None
        };
        let meta = if present {
            <Nbt<_>>::proto_decode(src).ok() // TODO: 握りつぶした
                                             // nbtタグが無い時にただの vec![0] が詰まっているように見える
                                             // nbtタグが有る時にもう一度確かめる
        } else {
            None
        };
        Ok(Self {
            present,
            item_id,
            item_count,
            meta,
        })
    }
}
