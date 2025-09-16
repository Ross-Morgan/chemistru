extern crate proc_macro;

use chemistru_elements_inner::{ELEMENTS, Element};
use proc_macro::TokenStream;
use quote::quote;

/// Generate a constant for each element
///
/// # Examples
///
/// ```
/// # use chemistru_elements_macro::generate_elements;
/// mod elements {
///     generate_elements!();
/// }
///
/// let hydrogen = elements::HYDROGEN;
/// ```
#[proc_macro]
pub fn generate_elements(_: TokenStream) -> TokenStream {
    let elements = ELEMENTS.iter();

    let mut stream = TokenStream::new();

    for element in elements {
        stream.extend(generate_const_init(element));
    }

    stream
}

fn generate_const_init(element: &Element) -> TokenStream {
    let assignment_name = element.name().to_uppercase();

    TokenStream::from(quote! {
        const #assignment_name: &'static ::chemistru_elements::Element = &'static #element;
    })
}
