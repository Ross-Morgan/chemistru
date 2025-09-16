extern crate proc_macro;

use chemistru_elements_inner::{ELEMENTS, Element};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn generate_elements_consts(_: TokenStream) -> TokenStream {
    let elements = ELEMENTS.iter();

    // let mut stream = TokenStream2::new();
    let mut stream = TokenStream::new();

    for element in elements {
        stream.extend(generate_const_init(element));
    }

    stream
}

fn generate_const_init(element: &Element) -> TokenStream {
    let assignment_name = element.name().to_uppercase();

    TokenStream::from(quote! {
        const #assignment_name: ::chemistru_elements::Element = #element;
    })
}
