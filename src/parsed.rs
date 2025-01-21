use proc_macro2::TokenStream as TokenStream2;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};

pub struct Grammar {
    pub rules: Punctuated<Rule, Token![;]>,
}

pub struct Rule {
    pub name: syn::Ident,
    pub _arrow: Token![->],
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: syn::Ident,
    pub _paren: Option<syn::token::Paren>,
    pub ty: Option<syn::Type>,
    pub separator: Separator,
}

#[derive(PartialEq, Eq)]
pub enum Separator {
    Comma,
    Pipe,
}

impl Parse for Grammar {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let rules = Punctuated::parse_terminated(input)?;
        Ok(Self { rules })
    }
}

impl Parse for Rule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let arrow = input.parse()?;
        let mut fields = Vec::new();

        while let Ok(field) = input.parse() {
            fields.push(field);
        }

        Ok(Self {
            name,
            _arrow: arrow,
            fields,
        })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let (paren, ty) = parse_type(input)?;
        let separator = input.parse()?;
        Ok(Self {
            name,
            _paren: paren,
            ty,
            separator,
        })
    }
}

fn parse_type(input: ParseStream) -> syn::Result<(Option<syn::token::Paren>, Option<syn::Type>)> {
    if input.peek(syn::token::Paren) {
        let content;
        let paren = syn::parenthesized!(content in input);
        let ty = content.parse()?;
        Ok((Some(paren), Some(ty)))
    } else {
        Ok((None, None))
    }
}

impl Parse for Separator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            Ok(Separator::Comma)
        } else if input.peek(Token![|]) {
            input.parse::<Token![|]>()?;
            Ok(Separator::Pipe)
        } else {
            Err(input.error("Expected '|' for enums or ',' for structs"))
        }
    }
}
