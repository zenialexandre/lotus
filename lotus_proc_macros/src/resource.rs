extern crate proc_macro;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive_resource_impl(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(token_stream as DeriveInput);
    let name: &syn::Ident = &derive_input.ident;

    let gen: proc_macro2::TokenStream = quote! {
        impl crate::core::ecs::resource::Resource for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                return self;
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                return self;
            }
        }
    };
    return gen.into();
}
