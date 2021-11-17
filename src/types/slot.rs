use std::io::{Read, Result};

use crate::protocol::ProtocolRead;

use super::{item_stack_meta::ItemStackMeta, nbt::Nbt, Var};

#[derive(Debug, Clone, Default)]
pub struct Slot {
    pub item_id: Var<i32>,
    pub item_count: u8,
    pub meta: Option<Nbt<ItemStackMeta>>,
}

impl ProtocolRead for Slot {
    fn proto_decode<S: Read>(src: &mut S) -> Result<Self> {
        let item_id = <Var<i32>>::proto_decode(src)?;
        let item_count = u8::proto_decode(src)?;

        let meta = <Nbt<_>>::proto_decode(src).ok();
        // TODO: 握りつぶした
        // nbtタグが無い時にただの vec![0] が詰まっているように見える
        // nbtタグが有る時にもう一度確かめる

        Ok(Self {
            item_id,
            item_count,
            meta,
        })
    }
}
