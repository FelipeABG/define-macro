use proc_macro::TokenStream;

mod representation;

#[proc_macro]
pub fn bnf(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as representation::Bnf);
    TokenStream::new()
}
