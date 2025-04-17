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
                        
                        // Check if the type is Option<T>
                        let (type_id, is_optional) = match &field.ty {
                            Type::Path(type_path) => {
                                let segments = &type_path.path.segments;
                                if segments.last().unwrap().ident == "Option" {
                                    // Extract inner type from Option<T>
                                    if let Type::Path(inner_type) = extract_option_inner_type(&field.ty) {
                                        (quote! { TypeId::of::<#inner_type>() }, true)
                                    } else {
                                        (quote! { TypeId::of::<#type_path>() }, true)
                                    }
                                } else {
                                    (quote! { TypeId::of::<#type_path>() }, false)
                                }
                            },
                            _ => panic!("Unsupported field type"),
                        };

                        quote! {
                            ConfigPath::new(
                                #field_name_str.to_string(),
                                #type_id,
                                #is_optional,
                            )
                        }
                    });

                    quote! {
                        vec![
                            #(#field_paths),*
                        ]
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