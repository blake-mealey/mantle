extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(ResourceGroup)]
pub fn derive_resource_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let variants: Vec<_> = match &input.data {
        Data::Enum(data) => data.variants.iter().collect(),
        _ => panic!("expected enum to derive ResourceGroup"),
    };

    let variant_idents = variants.iter().map(|variant| variant.ident.clone());
    let variant_idents2 = variant_idents.clone();
    let variant_idents3 = variant_idents.clone();
    let variant_idents4 = variant_idents.clone();
    let variant_idents5 = variant_idents.clone();
    let variant_idents6 = variant_idents.clone();
    let variant_idents7 = variant_idents.clone();

    let expanded = quote! {
        #[async_trait]
        impl ResourceGroup for #name {
            fn id(&self) -> &str {
                match self {
                    #(Self::#variant_idents(resource) => &resource.id),*
                }
            }

            fn has_outputs(&self) -> bool {
                match self {
                    #(Self::#variant_idents2(resource) => resource.outputs.is_some()),*
                }
            }

            fn dependency_ids(&self) -> Vec<&str> {
                match self {
                    #(Self::#variant_idents3(resource) => resource.dependency_ids()),*
                }
            }

            fn next(
                &self,
                previous_graph: &ResourceGraph,
                next_graph: &ResourceGraph,
            ) -> anyhow::Result<RbxResource> {
                match self {
                    #(Self::#variant_idents4(resource) => Ok(Self::#variant_idents4(#variant_idents4::next(
                        resource,
                        previous_graph.get(&resource.id),
                        next_graph.get_many(resource.dependency_ids()),
                    )?))),*
                }
            }

            async fn create(&mut self) -> anyhow::Result<()> {
                match self {
                    #(Self::#variant_idents5(resource) => resource.create().await),*
                }
            }

            async fn update(&mut self) -> anyhow::Result<()> {
                match self {
                    #(Self::#variant_idents6(resource) => resource.update().await),*
                }
            }

            async fn delete(&mut self) -> anyhow::Result<()> {
                match self {
                    #(Self::#variant_idents7(resource) => resource.delete().await),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Resource, attributes(dependency, resource_group))]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let data = match &input.data {
        Data::Struct(data) => data,
        _ => panic!("expected struct to derive Resource"),
    };

    let dependency_fields: Vec<_> = data
        .fields
        .iter()
        .filter_map(|field| {
            if field.attrs.iter().any(|a| a.path.is_ident("dependency")) {
                let var_name = field.ident.clone().unwrap();
                let field_type = if let syn::Type::Path(path) = &field.ty {
                    path.path.get_ident().unwrap().clone()
                } else {
                    panic!("expected dependency type to be a type path");
                };
                Some((var_name, field_type))
            } else {
                None
            }
        })
        .collect();
    let dependency_field_idents = dependency_fields
        .iter()
        .map(|(var_name, _field_type)| var_name);

    let dependency_variables = dependency_fields.iter().map(|(var_name, field_type)| {
        quote! {
            let mut #var_name: Option<#field_type> = None;
        }
    });

    let dependency_matchers = dependency_fields.iter().map(|(var_name, field_type)| {
        quote! {
            RbxResource::#field_type(resource) => {
                #var_name = Some(resource.clone());
            }
        }
    });

    let dependency_values = dependency_fields.iter().map(|(var_name, field_type)| {
        let field_type_str = field_type.to_string();
        quote! {
            #var_name: #var_name.ok_or(anyhow::Error::msg(format!(
                "Expected dependency of type {} to be present",
                #field_type_str
            )))?
        }
    });

    let expanded = quote! {
        impl Resource for #name {
            // TODO: RbxResource should come from a variable/attribute
            fn next(
                resource: &Self,
                previous_resource: Option<&RbxResource>,
                dependencies: Vec<&RbxResource>
            ) -> anyhow::Result<Self> {
                #(#dependency_variables)*

                for dependency in dependencies {
                    match dependency {
                        #(#dependency_matchers)*
                        _ => {}
                    }
                }

                let outputs = match previous_resource {
                    Some(RbxResource::#name(resource)) => {
                        resource.outputs.clone()
                    }
                    Some(_) => {
                        return anyhow::Result::Err(anyhow::Error::msg(format!(
                            "Expected previous resource with ID {} to be of the same type",
                            resource.id
                        )))
                    }
                    None => None
                };

                Ok(Self {
                    id: resource.id.clone(),
                    inputs: resource.inputs.clone(),
                    outputs,
                    #(#dependency_values),*
                })
            }

            fn dependency_ids(&self) -> Vec<&str> {
                vec![
                    #(&self.#dependency_field_idents.id),*
                ]
            }
        }
    };

    TokenStream::from(expanded)
}
