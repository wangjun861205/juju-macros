use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ToTuple)]
pub fn to_tuple(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let mut types = quote!();
    match &input.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(fs) => {
                for f in &fs.named {
                    let typ = f.ty.clone();
                    types.extend(quote!(#typ,))
                }
            }
            _ => {
                panic!("only named fields are supported")
            }
        },
        _ => panic!("only structs are supported"),
    };
    let name = format_ident!("{}Row", input.ident);
    let output = quote!(pub type #name = (#types););
    output.into()
}
