use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, FieldsNamed, GenericArgument, Path, PathArguments, Type,
};

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
