extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use self::proc_macro::{TokenStream};
use syn::{Item, Ident};
use syn::export::Span;

#[proc_macro_attribute]
pub fn ffi(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).expect("failed to parse input into `syn::Item`");

    let output = match item {
        Item::Struct(ref struct_item) => { 
            let struct_name = &struct_item.ident;
            let constructor_ident = Ident::new(&format!("ffi_{}_new", struct_item.ident), Span::call_site());
            let destructor_ident = Ident::new(&format!("ffi_{}_free", struct_item.ident), Span::call_site());

            let fields: Option<syn::punctuated::Punctuated<syn::Field, syn::token::Comma>> = match struct_item.fields {
                syn::Fields::Named(ref named_fields) => Some(named_fields.named.clone()),
                _ => None,
            };

            quote!{
                #item 

                pub unsafe extern fn #constructor_ident(#(#fields),*) -> *mut #struct_name {
                    let boxed_var = Box::new(#struct_name {

                    });
                    Box::into_raw(boxed_var)
                }

                pub unsafe extern fn #destructor_ident(ptr: *mut #struct_name) {
                    let _ = Box::from_raw(ptr);
                }
            }
        },
        _ => quote!{ #item },
    };

    println!("{}", output);

    output.into()
}

fn gen_struct_ffi(struct_item: &syn::ItemStruct) -> TokenStream {
    
}