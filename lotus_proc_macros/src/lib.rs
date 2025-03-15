mod component;
mod resource;

extern crate proc_macro;

#[proc_macro_derive(Component)]
pub fn derive_component(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    return component::derive_component_impl(token_stream);
}

#[proc_macro_derive(Resource)]
pub fn derive_resource(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    return resource::derive_resource_impl(token_stream);
}
