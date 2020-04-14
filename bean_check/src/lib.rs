extern crate proc_macro;
extern crate regex;

use regex::Regex;
use crate::proc_macro::TokenStream;

use quote::quote;
use syn;
use proc_macro::Ident;
use std::str::FromStr;

///
///
/// NotEmpty : not null or empty
/// Length(1,3) : String's length between (min, max)
/// Min(18) : integer min value
/// Max(20) : integer max value
/// Range(2,5) : integer value between (min, max)
/// Email : is email address or not
/// Pattern(r"^.*$") : regular expression
///
#[proc_macro_derive(BeanCheck, attributes(NotEmpty, Length, Min, Max, Range, Email, Pattern))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let mut aa = "";

    let mut check_quote = quote! {};

    match &ast.data {
        syn::Data::Struct(ds) => {
            match &ds.fields {
                syn::Fields::Named(ff) => {
                    ff.named.iter().for_each(|f| {
                        let ident = &f.ident;
                        let ty = &f.ty;
                        let attrs = &f.attrs;

                        let mut ft = String::from("");

                        match ty {
                            syn::Type::Path(p) => {
                                ft = format!("{}", p.path.segments[0].ident);
                            },
                            _ => println!("none"),
                        }

                        println!("field type is {}", ft);

                        attrs.iter().for_each(|at| {

                            // println!("-- {} -->{} -> {}", at.path.segments.is_empty(), at.path.segments.len(), at.path.segments[0].ident);
                            // println!("-----> {} : {}", at.tokens.is_empty(), at.tokens);

                            let mut prop = String::from("");
                            let mut param = String::from("");

                            if !at.path.segments.is_empty() && at.path.segments.len() > 0 {
                                prop = format!("{}", at.path.segments[0].ident);
                            }

                            if !at.tokens.is_empty() {
                                param = format!("{}", at.tokens);
                            }

                            println!("field type  {} - [{} - {}]", ft, prop, param);

                            // handle String field
                            if ft == "String".to_owned() {
                                let tmp = param.replace("(", "").replace(")", "").replace(" ", "");
                                // handle property : Length(1,2)
                                if prop == "Length".to_string() && param.len() > 0 {
                                    let mut min = 0usize;
                                    let mut max = 0usize;

                                    let rst:Vec<&str> = tmp.split(",").collect();
                                    if rst.len() == 2 {
                                        min = u32::from_str(rst[0]).unwrap_or(0) as usize;
                                        max = u32::from_str(rst[1]).unwrap_or(0) as usize;
                                    }
                                    println!("length : {}-{}", min, max);

                                    if check_quote.is_empty() {
                                        check_quote = quote! {
                                            (  self.#ident.len() >= #min && self.#ident.len() <= #max )
                                        }
                                    }
                                    else {
                                        check_quote = quote! {
                                            #check_quote &&
                                            ( self.#ident.len() >= #min && self.#ident.len() <= #max )
                                        }
                                    }
                                } else if prop == "Pattern".to_string() && param.len() > 0 {
                                    // handle property : Pattern(r"^.*$")
                                    if check_quote.is_empty() {
                                        check_quote = quote! {
                                            (  Regex::new(#tmp).unwrap().is_match(self.#ident.as_str()) )
                                        }
                                    }
                                    else {
                                        check_quote = quote! {
                                            #check_quote &&
                                            (  Regex::new(#tmp).unwrap().is_match(self.#ident.as_str()) )
                                        }
                                    }
                                }
                            } else if ft == "u8".to_string() || ft == "i8".to_string() || ft == "u16".to_string() || ft == "i16".to_string() || ft == "u32".to_string() || ft == "i32".to_string() || ft == "u64".to_string() || ft == "i64".to_string() || ft == "u128".to_string() || ft == "i128".to_string() || ft == "f32".to_string() || ft == "f64".to_string() {
                                // handle Integer field
                                println!("this is a integer");
                            }
                        });
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
            impl BeanCheck for #name {
                fn is_valid(&self)-> bool {
                    #check_quote
                    // println!("-----------------------> {}",#aa);
                    // println!("Hello Macro! My name is {}", stringify!(#name));
                    // true
                }
            }
        }
    )
}
