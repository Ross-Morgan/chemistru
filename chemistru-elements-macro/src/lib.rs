extern crate proc_macro;

use chemistru_elements_inner::{Element, ELEMENTS};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro]
pub fn generate_elements_consts(_: TokenStream) -> TokenStream {
    let elements = ELEMENTS.iter();

    let mut stream = TokenStream2::new();

    for element in elements {
        stream.extend(generate_const_init(element));
    }

    TokenStream::from(stream)
}

fn generate_const_init(element: &Element) -> TokenStream2 {
    let assignment_name = element.name().to_uppercase();

    quote! {
        const #assignment_name: ::chemistru_elements::Element = #element;
    }
}
