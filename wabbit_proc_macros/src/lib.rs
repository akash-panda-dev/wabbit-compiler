use proc_macro::TokenStream;
use quote::quote;
use syn::Data;
use syn::Fields;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FromAST)]
pub fn fromast_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let data_enum = match data {
        Data::Enum(data_enum) => data_enum,
        _ => panic!("FromExpr can only be applied to enums"),
    };

    let from_impls = data_enum.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let fields = match &variant.fields {
            Fields::Unnamed(fields) => fields,
            _ => panic!("FromExpr can only be applied to tuple variants"),
        };

        if fields.unnamed.len() != 1 {
            panic!("FromExpr can only be applied to tuple variants with 1 field")
        }

        let field_type = &fields.unnamed.first().unwrap().ty;

        quote! {
            impl From<#field_type> for #ident {
                fn from(value: #field_type) -> Self {
                    Self::#variant_ident(value)
                }
            }
        }
    });

    let output = quote! {
        #(#from_impls)*
    };

    output.into()
}
