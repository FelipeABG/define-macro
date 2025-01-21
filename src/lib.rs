use parsed::{Grammar, Separator};
use proc_macro::TokenStream;
use quote::quote;

mod parsed;

#[proc_macro]
pub fn bnf(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as Grammar);

    if let Err(e) = check_input(&ast) {
        let error = e.to_compile_error();
        return quote! {#error}.into();
    }

    quote! {#ast}.into()
}

fn check_input(bnf: &Grammar) -> syn::Result<()> {
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

    //Check if the struct rules fields have types
    for rule in bnf.rules.iter() {
        let sep = &rule.fields[0].separator;
        if let Separator::Comma = sep {
            for field in rule.fields.iter() {
                if let None = field.ty {
                    let msg = "Comma separated rules must have types in its fields";
                    return Err(syn::Error::new(field.name.span(), msg));
                }
            }
        }
    }

    Ok(())
}
