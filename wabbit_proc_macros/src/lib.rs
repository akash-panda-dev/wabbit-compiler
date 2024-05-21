use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// TODO: Fix this to also account for generics in the AST nodes as well

#[proc_macro_derive(Expression)]
pub fn expression_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let visit_fn = format_ident!("visit_{}", name.to_string().to_lowercase());

    let expanded = quote! {
        impl<T> Visitable<T> for #name #generics {
            fn accept(&self, visitor: &dyn Visitor<T>) -> T {
                visitor.#visit_fn(self)
            }
        }

        impl<T> Expression<T> for #name #generics {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Statement)]
pub fn statement_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let visit_fn = format_ident!("visit_{}", name.to_string().to_lowercase());

    let expanded = quote! {
        impl<T> Visitable<T> for #name #generics {
            fn accept(&self, visitor: &dyn Visitor<T>) -> T {
                visitor.#visit_fn(self)
            }
        }

        impl<T> Statement<T> for #name #generics {}
    };

    TokenStream::from(expanded)
}
