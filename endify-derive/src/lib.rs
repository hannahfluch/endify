use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Endify)]
pub fn endify_derive(input: TokenStream) -> TokenStream {
    // parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let to_le_impl = gen_method(&input, "to_le");
    let to_be_impl = gen_method(&input, "to_be");
    let from_le_impl = gen_method(&input, "from_le");
    let from_be_impl = gen_method(&input, "from_be");

    let name = &input.ident;

    let expanded = quote! {
        impl #impl_generics endify::Endify for #name #ty_generics #where_clause {
                    #to_le_impl
                    #to_be_impl
                    #from_le_impl
                    #from_be_impl
        }
    };

    expanded.into()
}

/// Helper that generates the implementation of a given method.
fn gen_method(input: &DeriveInput, method_name: &str) -> proc_macro2::TokenStream {
    let method_ident = syn::Ident::new(method_name, proc_macro2::Span::call_site());
    match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            // named fields
            syn::Fields::Named(fields) => {
                let conversions = fields.named.iter().map(|field| {
                    let ident = field.ident.as_ref().unwrap();
                    quote! {
                        #ident: Endify::#method_ident(self.#ident)
                    }
                });
                quote! {
                    fn #method_ident(self) -> Self {
                        Self {
                            #(#conversions),*
                        }
                    }
                }
            }
            // tuple structs
            syn::Fields::Unnamed(fields) => {
                let conversions = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = syn::Index::from(i);
                    quote! {
                        Endify::#method_ident(self.#index)
                    }
                });
                quote! {
                    fn #method_ident(self) -> Self {
                        Self (
                            #(#conversions),*
                        )
                    }
                }
            }
            // unit structs
            syn::Fields::Unit => {
                quote! {
                    fn #method_ident(self) -> Self {
                        self
                    }
                }
            }
        },
        _ => unimplemented!("only structs are supported right now. Feel free to open a PR."),
    }
}
