extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Resource, attributes(dependency))]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let dependency_fields: Vec<_> = match &input.data {
        Data::Struct(data) => data
            .fields
            .iter()
            .filter(|field| field.attrs.iter().any(|a| a.path.is_ident("dependency")))
            .collect(),
        _ => panic!("expected struct to derive Resource"),
    };

    let deps = dependency_fields
        .iter()
        .filter_map(|d| d.ident.clone())
        .map(|ident| quote! {self.#ident.clone()});

    let expanded = quote! {
        impl Resource for #name {
            fn id(&self) -> &str {
                &self.id
            }

            fn inputs(&self) -> &dyn ResourceInputs {
                &self.inputs
            }

            fn outputs(&self) -> &dyn ResourceOutputs {
                &self.outputs
            }

            fn dependencies(&self) -> Vec<WeakResourceRef> {
                vec![
                    #(#deps),*
                ]
            }
        }
    };

    TokenStream::from(expanded)
}

// impl Resource for ExperienceResource {
//     fn id(&self) -> &str {
//         &self.id
//     }

//     fn inputs(&self) -> &dyn ResourceInputs {
//         &self.inputs
//     }

//     fn outputs(&self) -> &dyn ResourceOutputs {
//         &self.outputs
//     }

//     fn dependencies(&self) -> Vec<WeakResourceRef> {
//         vec![]
//     }
// }
