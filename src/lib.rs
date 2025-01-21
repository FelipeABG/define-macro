use proc_macro::TokenStream;
use quote::quote;
use representation::Grammar;

mod representation;

#[proc_macro]
pub fn bnf(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as Grammar);

    if let Err(e) = check_input(ast) {
        let error = e.to_compile_error();
        return quote! {#error}.into();
    }

    TokenStream::new()
}

fn check_input(bnf: Grammar) -> syn::Result<Grammar> {
    // Check if there is an empty rule.
    if let Some(rule) = bnf.rules.iter().find(|r| r.fields.is_empty()) {
        return Err(syn::Error::new(
            rule.name.span(),
            "Invalid Rule. Expected at least one symbol",
        ));
    }

    //Check if the separators are all the same in a rule
    for rule in bnf.rules.iter() {
        let sep = &rule.fields[0].separator;
        for field in rule.fields.iter() {
            if field.separator != *sep {
                return Err(syn::Error::new(
                    field.name.span(),
                    "Expected either '|' or  ',' in a rule, not both",
                ));
            }
        }
    }

    Ok(bnf)
}
