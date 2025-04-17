use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

#[proc_macro_derive(ConfigMetadata)]
pub fn derive_config_metadata(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let paths = match input.data {
        Data::Struct(data) => {
            match data.fields {
                Fields::Named(fields) => {
                    let field_paths = fields.named.iter().map(|field| {
                        let field_name = field.ident.as_ref().unwrap();
                        let field_name_str = field_name.to_string();

                        match &field.ty {
                            Type::Path(type_path) => {
                                let segments = &type_path.path.segments;
                                let last_segment = segments.last().unwrap();
                                
                                if last_segment.ident == "Option" {
                                    // Handle Option<T> - wrap in vec![]
                                    if let Type::Path(inner_type) = extract_option_inner_type(&field.ty) {
                                        quote! {
                                            vec![ConfigPath::new(
                                                #field_name_str.to_string(),
                                                TypeId::of::<#inner_type>(),
                                                true,
                                            )]
                                        }
                                    } else {
                                        quote! {
                                            vec![ConfigPath::new(
                                                #field_name_str.to_string(),
                                                TypeId::of::<#type_path>(),
                                                true,
                                            )]
                                        }
                                    }
                                } else {
                                    // Handle nested structs that implement ConfigMetadata
                                    let type_name = &last_segment.ident;
                                    quote! {
                                        {
                                            let mut paths = Vec::new();
                                            // Add direct field path
                                            paths.push(ConfigPath::new(
                                                #field_name_str.to_string(),
                                                TypeId::of::<#type_path>(),
                                                false,
                                            ));
                                            // Add nested paths with prefix
                                            for mut path in #type_name::get_paths() {
                                                path.path = format!("{}.{}", #field_name_str, path.path);
                                                paths.push(path);
                                            }
                                            paths
                                        }
                                    }
                                }
                            },
                            _ => panic!("Unsupported field type"),
                        }
                    });

                    quote! {
                        {
                            let mut all_paths = Vec::new();
                            #(
                                all_paths.extend(#field_paths);
                            )*
                            all_paths
                        }
                    }
                },
                _ => panic!("Only named fields are supported"),
            }
        },
        _ => panic!("ConfigMetadata can only be derived for structs"),
    };

    let expanded = quote! {
        impl ConfigMetadata for #name {
            fn get_paths() -> Vec<ConfigPath<'static>> {
                #paths
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_option_inner_type(ty: &Type) -> Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let syn::GenericArgument::Type(inner_type) = args.args.first().unwrap() {
                        return inner_type.clone();
                    }
                }
            }
        }
    }
    panic!("Not an Option type")
}