use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, FieldsNamed, GenericArgument, Lit, Path, PathArguments,
    Type,
};

#[proc_macro_derive(PacketWrite)]
pub fn derive_packet_write(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl PacketWrite for #name {}
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(ProtocolRead)]
pub fn derive_protocol_read(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
        if let Some(vec_ty) = extract_vec_type(ty) {
            quote! {
                #ident: <Arr<Var<i32>, #vec_ty>>::proto_decode(src)?,
            }
        } else {
            quote! {
                #ident: <#ty>::proto_decode(src)?,
            }
        }
    });
    quote! {
        #(#optionateds)*
    }
}

#[proc_macro_derive(ProtocolWrite, attributes(packet_id))]
pub fn derive_protocol_write(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let encodes = proto_encode_fields(&input.data);

    let attribute = input
        .attrs
        .iter()
        .filter(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "packet_id")
        .nth(0)
        .expect("Expected #[packet_id = id]");

    let meta = attribute
        .parse_meta()
        .expect("Expected packet_id attribute");
    let packet_id = match meta {
        syn::Meta::NameValue(ref nv) => {
            if let Lit::Int(ref n) = nv.lit {
                n.base10_parse::<i32>().expect("Expected number id")
            } else {
                panic!("Expected 'packet_id = number'")
            }
        }
        _ => panic!("Expected 'packet_id = number'"),
    };

    let expanded = quote! {
        impl #name {
            #[inline(always)]
            fn packet_id() -> i32 {
                #packet_id
            }
        }

        impl ProtocolWrite for #name {
            fn proto_encode(value: &Self, dst: &mut dyn Write) -> std::io::Result<()> {
                <Var<i32>>::proto_encode(&Self::packet_id().into(), dst)?;
                #encodes

                Ok(())
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn proto_encode_fields(data: &Data) -> TokenStream {
    let fields = extract_fields(data);
    let optionateds = fields.named.iter().map(|f| {
        let ty = &f.ty;
        let ident = &f.ident;
        if let Some(vec_ty) = extract_vec_type(ty) {
            quote! {
                <Arr<Var<i32>, #vec_ty>>::proto_encode(&value.#ident, dst)?;
            }
        } else {
            quote! {
                <#ty>::proto_encode(&value.#ident, dst)?;
            }
        }
    });
    quote! {
        #(#optionateds)*
    }
}

fn extract_vec_type(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Path(typepath) if typepath.qself.is_none() && path_is_vec(&typepath.path) => {
            let type_params = &typepath.path.segments.iter().next().unwrap().arguments;
            // It should have only on angle-bracketed param ("<String>"):
            let generic_arg = match type_params {
                PathArguments::AngleBracketed(params) => params.args.iter().next().unwrap(),
                _ => panic!("TODO: error handling"),
            };
            // This argument must be a type:
            match generic_arg {
                GenericArgument::Type(ty) => Some(ty),
                _ => panic!("TODO: error handling"),
            }
        }
        _ => None,
    }
}

fn path_is_vec(path: &Path) -> bool {
    path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments.iter().next().unwrap().ident == "Vec"
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
