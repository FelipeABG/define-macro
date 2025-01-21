use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Result, Token,
};

pub struct Grammar {
    rules: Punctuated<Rule, Token![;]>,
}

pub struct Rule {
    pub kw: syn::Ident,
    pub name: syn::Ident,
    pub _arrow: Token![->],
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: syn::Ident,
    pub _paren: Option<syn::token::Paren>,
    pub ty: Option<syn::Type>,
}

impl ToTokens for Grammar {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let rules: Vec<&Rule> = self.rules.iter().collect();
        tokens.extend(quote! {#(#rules)*});
    }
}

impl ToTokens for Rule {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let kw = &self.kw;
        let name = syn::Ident::new(
            &format!("{}", capitalize(&self.name.to_string())),
            self.name.span(),
        );
        let fields = format_fields(self);
        let ts = quote! {
            pub #kw #name {
                #(#fields)*
            }
        };

        tokens.extend(ts);

        if kw.to_string() == "struct" {
            let args: Vec<_> = self
                .fields
                .iter()
                .map(|field| {
                    let name = &field.name;
                    let ty = &field.ty.clone().unwrap();
                    quote! {#name: #ty}
                })
                .collect();
            let names: Vec<_> = self.fields.iter().map(|f| &f.name).collect();
            tokens.extend(quote! {
                impl #name {
                    fn new(#(#args),*) -> Self {
                        Self {#(#names),*}
                    }
                }
            });
        }
    }
}

fn format_fields(rule: &Rule) -> Vec<TokenStream2> {
    let mut result = Vec::new();
    for field in &rule.fields {
        if rule.kw.to_string() == "struct" {
            let name = &field.name;
            let ty = &field.ty.clone().unwrap();
            result.push(quote! {pub #name: #ty,});
        } else {
            let name = syn::Ident::new(
                &format!("{}", capitalize(&field.name.to_string())),
                field.name.span(),
            );
            match &field.ty {
                Some(ty) => result.push(quote! {#name(#ty),}),
                None => result.push(quote! {#name,}),
            }
        }
    }

    result
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

impl Parse for Grammar {
    fn parse(input: ParseStream) -> Result<Self> {
        let rules = Punctuated::parse_terminated(input)?;
        Ok(Self { rules })
    }
}

impl Parse for Rule {
    fn parse(input: ParseStream) -> Result<Self> {
        let kw = input.call(syn::Ident::parse_any)?;
        let name = input.parse()?;
        let arrow = input.parse()?;
        let mut fields = Vec::new();

        while !input.peek(Token![;]) {
            fields.push(input.parse()?);
        }

        Ok(Self {
            kw,
            name,
            _arrow: arrow,
            fields,
        })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let (paren, ty) = parse_type(input)?;
        parse_separator(input)?;
        Ok(Self {
            name,
            _paren: paren,
            ty,
        })
    }
}

fn parse_separator(input: ParseStream) -> Result<()> {
    if input.peek(Token![,]) {
        let _: Token![,] = input.parse()?;
    } else if input.peek(Token![|]) {
        let _: Token![|] = input.parse()?;
    }

    Ok(())
}

fn parse_type(input: ParseStream) -> Result<(Option<syn::token::Paren>, Option<syn::Type>)> {
    if input.peek(syn::token::Paren) {
        let content;
        let paren = parenthesized!(content in input);
        let ty = content.parse()?;
        Ok((Some(paren), Some(ty)))
    } else {
        Ok((None, None))
    }
}
