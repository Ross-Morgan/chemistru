use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

use crate::{
    Element,
    atomic::AtomicData,
    electron::{ElectronConfiguration, ElectronData, Suborbital},
    misc::{Category, MiscData},
    physical::PhysicalData,
    table::TableData,
};

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name;
        let symbol = self.symbol;
        let atomic_data = self.atomic_data;
        let electron_data = self.electron_data;
        let physical_data = self.physical_data;
        let table_data = self.table_data;
        let misc_data = self.misc_data;

        tokens.extend(quote! {
            crate::Element::new(#name, #symbol, #atomic_data, #electron_data, #physical_data, #table_data, #misc_data)
        });
    }
}

impl ToTokens for AtomicData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let atomic_mass = self.atomic_mass;
        let atomic_number = self.atomic_number;
        let mass_number = self.mass_number;

        tokens.extend(quote! {
            crate::data::AtomicData::new(#atomic_mass, #atomic_number, #mass_number)
        });
    }
}

impl ToTokens for ElectronData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let configuration = self.configuration;
        let affinity = quote_option(self.affinity);
        let electronegativity = quote_option(self.electronegativity);
        let ionization_energies =
            quote_option(self.ionization_energies.map(|v| quote! { &[ #(#v),* ] }));
        let shells = self.shells;
        let shells = quote! { &[ #(#shells),* ] };

        tokens.extend(quote! {
            crate::data::ElectronData::new(#configuration, #affinity, #electronegativity, #ionization_energies, #shells)
        });
    }
}

impl ToTokens for ElectronConfiguration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let suborbital = self.suborbitals().iter();

        tokens.extend(quote! {
            crate::data::ElectronConfiguration::new(&[#(#suborbital),*])
        });
    }
}

impl ToTokens for Suborbital {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let principal_quantum_number = self.principal_quantum_number();
        let azimuthal_quantum_number = self.azimuthal_quantum_number();
        let electron_number = self.electron_number();

        tokens.extend(quote! {
            crate::data::Suborbital::new(#principal_quantum_number, #azimuthal_quantum_number, #electron_number)
        });
    }
}

impl ToTokens for PhysicalData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let boiling_point = quote_option(self.boiling_point);
        let melting_point = quote_option(self.melting_point);
        let density = quote_option(self.density);
        let molar_heat_capacity = quote_option(self.molar_heat_capacity);
        let phase_in_standard_conditions = self.phase_in_standard_conditions;

        tokens.extend(quote! {
            crate::data::PhysicalData::new(#boiling_point, #melting_point, #density, #molar_heat_capacity, #phase_in_standard_conditions)
        });
    }
}

impl ToTokens for TableData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (x, y) = self.position;

        tokens.extend(quote! { crate::data::TableData::new((#x, #y)) });
    }
}

impl ToTokens for MiscData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let appearance = quote_option(self.appearance);
        let category = self.category;
        let discovered_by = quote_option(self.discovered_by);
        let named_by = quote_option(self.named_by);
        let spectral_img = quote_option(self.spectral_img);
        let source = self.source;
        let cpk_color = quote_option(self.cpk_color);

        tokens.extend(quote! {
            crate::data::MiscData::new(#appearance, #category, #discovered_by, #named_by, #spectral_img, #source, #cpk_color)
        });
    }
}

impl ToTokens for Category {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant_name: &'static str = self.into();
        let variant_name = format_ident!("{variant_name}");

        tokens.extend(quote! { crate::data::Category::#variant_name });
    }
}

fn quote_option<T: ToTokens>(value: Option<T>) -> proc_macro2::TokenStream {
    match value {
        None => quote! { None },
        Some(v) => quote! { Some(#v) },
    }
}
