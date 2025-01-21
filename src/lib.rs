use parsed::Grammar;
use proc_macro::TokenStream;
use quote::quote;

mod parsed;

#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as Grammar);
    quote! {#ast}.into()
}
