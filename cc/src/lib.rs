extern crate proc_macro;

use crate::proc_macro::TokenStream;

use quote::quote;
use syn;

#[proc_macro_derive(Hello, attributes(range, length, reg, size, nullable))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let mut aa = "";
    let mut tmp = quote! {};

    match &ast.data {
        syn::Data::Struct(ds) => {
            match &ds.fields {
                syn::Fields::Named(ff) => {
                    ff.named.iter().for_each(|f| {
                        let ident = &f.ident;
                        let ty = &f.ty;
                        let attrs = &f.attrs;

                        attrs.iter().for_each(|at| {
                            println!("-- {} -->{} -> {}", at.path.segments.is_empty(), at.path.segments.len(), at.path.segments[0].ident);
                            println!("-----> {} : {}", at.tokens.is_empty(), at.tokens);
                        });

                        let m = quote! {
                            println!("field  {} ---> {}", stringify!(#ident), stringify!(#ty));
                        };

                        // fa.push(TokenStream::from(m));
                        tmp = quote! {
                            #tmp
                            #m
                        }
                    });
                    aa = "1111111111"
                }
                _ => {
                    aa = "222"
                }
            }
        }
        _ => {
            aa = "3333"
        }
    }

    TokenStream::from(
        quote! {
            impl Hello for #name {
                fn hi() {
                    #tmp
                    println!("-> {}",#aa);
                    println!("Hello Macro! My name is {}", stringify!(#name));
                }
            }
        }
    )
}

