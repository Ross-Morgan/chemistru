use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

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
        let atomic_data = self.atomic_data.clone();
        let electron_data = self.electron_data.clone();
        let physical_data = self.physical_data.clone();
        let table_data = self.table_data;
        let misc_data = self.misc_data.clone();

        tokens.extend(quote! {
            ::chemistru_elements::Element::new(#name, #symbol, #atomic_data, #electron_data, #physical_data, #table_data, #misc_data)
        });
    }
}

impl ToTokens for AtomicData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let atomic_mass = self.atomic_mass;
        let atomic_number = self.atomic_number;
        let mass_number = self.mass_number;

        tokens.extend(quote! {
            ::chemistru_elements::data::atomic::AtomicData::new(#atomic_mass, #atomic_number, #mass_number)
        });
    }
}

impl ToTokens for ElectronData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let configuration = self.configuration.clone();
        let affinity = self.affinity;
        let electronegativity = self.electronegativity;
        let ionization_energies = self
            .ionization_energies
            .clone()
            .map(|v| quote! { &'static [ #(#v),* ] });
        let shells = self.shells.clone();
        let shells = quote! { &'static [ #(#shells),* ] };

        tokens.extend(quote! {
            ::chemistru_elements::data::electron::ElectronData::new(#configuration, #affinity, #electronegativity, #ionization_energies, #shells)
        });
    }
}

impl ToTokens for ElectronConfiguration {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let suborbital = self.suborbitals().iter();

        tokens.extend(quote! {
            ::chemistru_elements::data::electron::ElectronConfiguration::new(&'static [#(#suborbital),*])
        });
    }
}

impl ToTokens for Suborbital {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let principal_quantum_number = self.principal_quantum_number();
        let azimuthal_quantum_number = self.azimuthal_quantum_number();
        let electron_number = self.electron_number();

        tokens.extend(quote! {
            quote! { ::chemistru_elements::data::electron::Suborbital::new(#principal_quantum_number, #azimuthal_quantum_number, #electron_number) }
        });
    }
}

impl ToTokens for PhysicalData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let boiling_point = self.boiling_point;
        let melting_point = self.melting_point;
        let density = self.density;
        let molar_heat_capacity = self.molar_heat_capacity;
        let phase_in_standard_conditions = self.phase_in_standard_conditions;

        tokens.extend(quote! {
            ::chemistru_elements::data::physical::PhysicalData::new(#boiling_point, #melting_point, #density, #molar_heat_capacity, #phase_in_standard_conditions)
        });
    }
}

impl ToTokens for TableData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (x, y) = self.position;

        tokens.extend(quote! { ::chemistru_elements::data::table::TableData::new((#x, #y)) });
    }
}

impl ToTokens for MiscData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let appearance = self.appearance;
        let category = self.category.clone();
        let discovered_by = self.discovered_by;
        let named_by = self.named_by;
        let spectral_img = self.spectral_img;
        let source = self.source;
        let cpk_color = self.cpk_color;

        tokens.extend(quote! {
            ::chemistru_elements::data::misc::MiscData::new(#appearance, #category, #discovered_by, #named_by, #spectral_img, #source, #cpk_color)
        });
    }
}

impl ToTokens for Category {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant_name: &'static str = self.into();

        match self {
            Self::Unknown { predicted } => {
                let inner_variant = predicted.as_ref().clone();
                tokens.extend(quote! { ::chemistru_elements::data::misc::Category::Unknown { predicted: ::std::boxed::Box::new(#inner_variant) }});
            }
            _ => {
                tokens.extend(quote! { ::chemistru_elements::data::misc::Category::#variant_name });
            }
        }
    }
}
