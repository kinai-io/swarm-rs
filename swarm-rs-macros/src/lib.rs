use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemImpl};

#[proc_macro_attribute]
pub fn agent_action(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input.into()
}

#[proc_macro_attribute]
pub fn agent_workflow(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input.into()
}

#[proc_macro_attribute]
pub fn agent(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let impl_decl = parse_macro_input!(input_clone as ItemImpl);

    let struct_ident = if let syn::Type::Path(type_path) = impl_decl.self_ty.as_ref() {
        let ident: &Ident = type_path.path.get_ident().unwrap();
        // println!("\t- Name: \"{:?}\"", ident);
        ident
    } else {
        panic!("Expected a struct with named fields");
    };

    let mut match_arms: Vec<proc_macro2::TokenStream> = vec![];
    for item in &impl_decl.items {
        if let syn::ImplItem::Fn(method) = item {
            for attr in &method.attrs {
                // Include function into dipatch
                if attr.meta.path().is_ident("agent_action") {
                    let fn_ident = method.sig.ident.clone();
                    let match_arm = quote! {
                        stringify!(#fn_ident) => {
                            if let Ok(input_value) = action.get_payload() {
                                let output = self.#fn_ident(input_value).await;
                                match output {
                                    Ok(output_value) => Output::new_success(output_value),
                                    Err(message) => Output::new_error(&message)
                                }
                            }else {
                                Output::new_error("Unable to get payload")
                            }
                        },
                    };
                    match_arms.push(match_arm);
                } else if attr.meta.path().is_ident("agent_workflow") {
                    let fn_ident = method.sig.ident.clone();
                    let match_arm = quote! {
                        stringify!(#fn_ident) => {
                            if let Ok(input_value) = action.get_payload() {
                                let output = self.#fn_ident(input_value, swarm).await;
                                match output {
                                    Ok(output_value) => Output::new_success(output_value),
                                    Err(message) => Output::new_error(&message)
                                }
                            }else {
                                Output::new_error("Unable to get payload")
                            }
                        },
                    };
                    match_arms.push(match_arm);
                }
            }
        }
    }

    let expanded: proc_macro2::TokenStream = quote! {

        #impl_decl

        #[async_trait]
        impl Agent for #struct_ident {

            fn as_any(&self) -> &dyn Any {
                self
            }

            async fn execute(&self, action: &Action, swarm: &Swarm) -> Output {
                match action.get_name() {
                    #(#match_arms)*
                    _ => Output::new_error("Unknown action"),
                }
            }
        }

    };

    expanded.into()
}
