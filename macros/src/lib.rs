use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, FieldsNamed};

#[proc_macro_derive(ProtocolRead)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let decodes = proto_decode_fields(&input.data);

    let expanded = quote! {
        impl ProtocolRead for #name {
            fn proto_decode(src: &mut dyn std::io::Read) -> std::io::Result<Self> {
                Ok(Self {
                    #decodes
                })
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn proto_decode_fields(data: &Data) -> TokenStream {
    let fields = extract_fields(data);
    let optionateds = fields.named.iter().map(|f| {
        let ty = &f.ty;
        let ident = &f.ident;
        quote! {
            #ident: <#ty>::proto_decode(src)?,
        }
    });
    quote! {
        #(#optionateds)*
    }
}

fn extract_fields(data: &Data) -> &FieldsNamed {
    match *data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields_named) => fields_named,
            _ => panic!("expected all fields which named"),
        },
        _ => panic!("Struct expected!"),
    }
}
