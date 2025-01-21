use parsed::Grammar;
use proc_macro::TokenStream;

mod parsed;

#[proc_macro]
pub fn bnf(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as Grammar);
    TokenStream::new()
}

fn check_input(bnf: &Grammar) -> syn::Result<()> {
    todo!()
}
