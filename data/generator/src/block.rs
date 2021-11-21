use std::{fs, io::Result};

use convert_case::{Case, Casing};
use proc_macro2::Literal;
use quote::{format_ident, quote};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct BlockData {
    id: u64,
    display_name: String,
    name: String,
    hardness: Option<f32>,
    min_state_id: u64,
    max_state_id: u64,
    // "states": [],
    drops: Vec<u64>,
    diggable: bool,
    transparent: bool,
    filter_light: u64,
    emit_light: u64,
    // "boundingBox": "empty",
    stack_size: u64,
    default_state: u64,
    resistance: f32,
}

pub fn generate_block(version: &str) -> Result<String> {
    let blocks_json =
        fs::read_to_string(format!("minecraft-data/data/pc/{}/blocks.json", version))?;
    let blocks: Vec<BlockData> = serde_json::from_str(&blocks_json)?;
    println!("block sizes: {}", blocks.len());

    let block_enum_names = blocks
        .iter()
        .map(|i| format_ident!("{}", i.name.to_case(Case::Pascal)))
        .collect::<Vec<_>>();
    let block_enum = quote! {
        pub enum Block {
            #(
                #block_enum_names,
            )*
        }
    };

    let mut fns = vec![];

    let block_to_ids = blocks.iter().map(|block| {
        let enum_name = format_ident!("{}", block.name.to_case(Case::Pascal));
        let id = Literal::u64_unsuffixed(block.id);

        quote! {
            Block::#enum_name => #id
        }
    });
    fns.push(quote! {
        pub fn id(&self) -> u32 {
            match self {
                #(#block_to_ids,)*
            }
        }
    });

    let id_from_blocks = blocks.iter().map(|block| {
        let enum_name = format_ident!("{}", block.name.to_case(Case::Pascal));
        let id = Literal::u64_unsuffixed(block.id);

        quote! {
            #id => Some(Block::#enum_name)
        }
    });
    fns.push(quote! {
        pub fn from_id(id: u32) -> Option<Self> {
            match id {
                #(#id_from_blocks,)*
                _ => None,
            }
        }
    });

    let block_to_names = blocks.iter().map(|block| {
        let enum_name = format_ident!("{}", block.name.to_case(Case::Pascal));
        let name = Literal::string(&block.name);

        quote! {
            Block::#enum_name => #name.to_string()
        }
    });
    fns.push(quote! {
        pub fn name(&self) -> String {
            match self {
                #(#block_to_names,)*
            }
        }
    });

    let name_from_blocks = blocks.iter().map(|block| {
        let enum_name = format_ident!("{}", block.name.to_case(Case::Pascal));
        let name = Literal::string(&block.name);

        quote! {
            #name => Some(Block::#enum_name)
        }
    });
    fns.push(quote! {
        pub fn from_name(name: &str) -> Option<Self> {
            match name {
                #(#name_from_blocks,)*
                _ => None,
            }
        }
    });

    let block_to_display_names = blocks.iter().map(|block| {
        let enum_name = format_ident!("{}", block.name.to_case(Case::Pascal));
        let display_name = Literal::string(&block.display_name);

        quote! {
            Block::#enum_name => #display_name.to_string()
        }
    });
    fns.push(quote! {
        pub fn display_name(&self) -> String {
            match self {
                #(#block_to_display_names,)*
            }
        }
    });

    let block_to_default_states = blocks.iter().map(|block| {
        let enum_name = format_ident!("{}", block.name.to_case(Case::Pascal));
        let default_state = Literal::u64_unsuffixed(block.default_state);

        quote! {
            Block::#enum_name => #default_state
        }
    });
    fns.push(quote! {
        pub fn stack_size(&self) -> u32 {
            match self {
                #(#block_to_default_states,)*
            }
        }
    });

    let block_fns = quote! {
        impl Block {
            #(#fns)*
        }
    };

    Ok(format!(
        "{}\n\n{}",
        block_enum.to_string(),
        block_fns.to_string()
    ))
}
