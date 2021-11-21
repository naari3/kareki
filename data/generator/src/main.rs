use std::{
    fs::File,
    io::{Result, Write},
    process::Command,
};

use block::generate_block;
use item::generate_item;

mod block;
mod item;

fn main() -> Result<()> {
    let version = "1.15.2";

    let item_rs = generate_item(version)?;
    let mut item_rs_file = File::create("../src/item.rs")?;
    item_rs_file.write_all(item_rs.as_bytes())?;

    let block_rs = generate_block(version)?;
    let mut block_rs_file = File::create("../src/block.rs")?;
    block_rs_file.write_all(block_rs.as_bytes())?;

    Command::new("cargo")
        .current_dir("../")
        .args(&["fmt"])
        .output()?;

    Ok(())
}
