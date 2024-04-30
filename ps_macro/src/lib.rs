extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(DataResource)]
pub fn data_resource_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct or enum being derived for
    let name = &ast.ident;

    // Generate match arms for each variant in the enum (if it's an enum)
    let match_arms = match ast.data {
        Data::Enum(ref data_enum) => {
            data_enum
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    match &variant.fields {
                        Fields::Unit => {
                            // Convert the variant identifier to a string
                            let variant_name = variant_ident.to_string();
                            // If the variant name has multiple words, separate them with underscores
                            let variant_name_with_underscores =
                                variant_name.chars().fold(String::new(), |mut acc, c| {
                                    if c.is_uppercase() {
                                        if !acc.is_empty() {
                                            acc.push('_');
                                        }
                                        acc.push(c.to_lowercase().next().unwrap());
                                    } else {
                                        acc.push(c);
                                    }
                                    acc
                                });
                            // Generate the match arm
                            quote! {
                                #name::#variant_ident => #variant_name_with_underscores.to_string(),
                            }
                        }
                        _ => {
                            panic!("DataResource can only be derived for enums with unit variants")
                        }
                    }
                })
                .collect::<Vec<_>>()
        }
        _ => Vec::new(), // No match arms for non-enum types
    };

    // Generate the implementation for the trait DataResource
    let expanded = quote! {
        impl DataResource for #name {
            fn as_api(&self) -> String {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    // Return the generated implementation as a TokenStream
    TokenStream::from(expanded)
}
