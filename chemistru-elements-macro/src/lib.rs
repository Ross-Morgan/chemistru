extern crate proc_macro;

use chemistru_elements_inner::{ELEMENTS, Element};
use proc_macro::TokenStream;
use quote::{format_ident, quote};

/// Generate a constant for each element
///
/// # Examples
///
/// ```
/// mod elements {
/// # use crate::generate_elements;
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
    let assignment_name = format_ident!("{}", element.name().to_uppercase());

    TokenStream::from(quote! {
        const #assignment_name: &'static crate::Element = &#element;
    })
}
