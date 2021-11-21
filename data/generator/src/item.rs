use std::{fs, io::Result};

use convert_case::{Case, Casing};
use proc_macro2::Literal;
use quote::{format_ident, quote};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ItemData {
    id: u64,
    display_name: String,
    name: String,
    stack_size: u64,
}

pub fn generate_item(version: &str) -> Result<String> {
    let items_json = fs::read_to_string(format!("minecraft-data/data/pc/{}/items.json", version))?;
    let mut items: Vec<ItemData> = serde_json::from_str(&items_json)?;
    items.insert(
        0,
        ItemData {
            id: 0,
            display_name: "Air".into(),
            name: "air".into(),
            stack_size: 0,
        },
    );
    println!("item sizes: {}", items.len());

    let item_enum_names = items
        .iter()
        .map(|i| format_ident!("{}", i.name.to_case(Case::Pascal)))
        .collect::<Vec<_>>();
    let item_enum = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Item {
            #(
                #item_enum_names,
            )*
        }
    };

    let mut fns = vec![];

    let item_to_ids = items.iter().map(|item| {
        let enum_name = format_ident!("{}", item.name.to_case(Case::Pascal));
        let id = Literal::u64_unsuffixed(item.id);

        quote! {
            Item::#enum_name => #id
        }
    });
    fns.push(quote! {
        pub fn id(&self) -> u32 {
            match self {
                #(#item_to_ids,)*
            }
        }
    });

    let id_from_items = items.iter().map(|item| {
        let enum_name = format_ident!("{}", item.name.to_case(Case::Pascal));
        let id = Literal::u64_unsuffixed(item.id);

        quote! {
            #id => Some(Item::#enum_name)
        }
    });
    fns.push(quote! {
        pub fn from_id(id: u32) -> Option<Self> {
            match id {
                #(#id_from_items,)*
                _ => None,
            }
        }
    });

    let item_to_names = items.iter().map(|item| {
        let enum_name = format_ident!("{}", item.name.to_case(Case::Pascal));
        let name = Literal::string(&item.name);

        quote! {
            Item::#enum_name => #name.to_string()
        }
    });
    fns.push(quote! {
        pub fn name(&self) -> String {
            match self {
                #(#item_to_names,)*
            }
        }
    });

    let name_from_items = items.iter().map(|item| {
        let enum_name = format_ident!("{}", item.name.to_case(Case::Pascal));
        let name = Literal::string(&item.name);

        quote! {
            #name => Some(Item::#enum_name)
        }
    });
    fns.push(quote! {
        pub fn from_name(name: &str) -> Option<Self> {
            match name {
                #(#name_from_items,)*
                _ => None,
            }
        }
    });

    let item_to_display_names = items.iter().map(|item| {
        let enum_name = format_ident!("{}", item.name.to_case(Case::Pascal));
        let display_name = Literal::string(&item.display_name);

        quote! {
            Item::#enum_name => #display_name.to_string()
        }
    });
    fns.push(quote! {
        pub fn display_name(&self) -> String {
            match self {
                #(#item_to_display_names,)*
            }
        }
    });

    let item_to_stack_sizes = items.iter().map(|item| {
        let enum_name = format_ident!("{}", item.name.to_case(Case::Pascal));
        let stack_size = Literal::u64_unsuffixed(item.stack_size);

        quote! {
            Item::#enum_name => #stack_size
        }
    });
    fns.push(quote! {
        pub fn stack_size(&self) -> u32 {
            match self {
                #(#item_to_stack_sizes,)*
            }
        }
    });

    let item_fns = quote! {
        impl Item {
            #(#fns)*
        }
    };

    Ok(format!(
        "{}\n\n{}",
        item_enum.to_string(),
        item_fns.to_string()
    ))
}
