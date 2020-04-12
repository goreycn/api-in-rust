extern crate proc_macro;

use crate::proc_macro::TokenStream;

use quote::quote;
use syn;

#[proc_macro_derive(Hello)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let mut aa = "";
    let mut field_arr:Vec<proc_macro::TokenStream> =  Vec::new();

    match &ast.data {
        syn::Data::Struct(ds) => {
            match &ds.fields {
                syn::Fields::Named(ff) => {
                    ff.named.iter().for_each(|f| {
                        let ident = &f.ident;
                        let ty = &f.ty;

                        let m = quote! {
                            println!("field ===> {} : {}", #ident, #ty);
                        };

                        field_arr.push(m);
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

    // #(#field_arr)*

    TokenStream::from(
        quote! {
            impl Hello for #name {
                fn hi() {
                    #(#field_arr)*
                    println!("-> {}",#aa);
                    // println!("=======> {}", #field_arr);
                    println!("Hello Macro! My name is {}", stringify!(#name));
                }
            }
        }
    )
    // gen.into()
}

