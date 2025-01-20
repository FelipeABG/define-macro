use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::Token;

//MENTAL MODEL
// bnf := type_name '->' body
// body := (field (',' | '|'))+
//

#[proc_macro]
pub fn bnf(input: TokenStream) -> TokenStream {
    TokenStream::new()
}

struct Bnf {
    type_name: syn::Ident,
    body: Body,
}

struct Body {
    fields: Vec<Field>,
}

struct Field {
    name: syn::Ident,
    separator: Separator,
}

enum Separator {
    Comma,
    Pipe,
}
