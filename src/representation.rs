use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Token,
};

pub struct Bnf {
    definitions: Punctuated<Definition, Token![;]>,
}

pub struct Definition {
    name: syn::Ident,
    arrow: Token![->],
    fields: Vec<Field>,
}

pub struct Field {
    name: syn::Ident,
    colon: Option<Token![:]>,
    ty: Option<syn::Type>,
    value_variant: Option<ValueVariant>,
    separator: Separator,
}

pub struct ValueVariant {
    paren: syn::token::Paren,
    ty: syn::Type,
}

pub enum Separator {
    Comma,
    Pipe,
}

impl Parse for Bnf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let definitions = Punctuated::parse_terminated(input)?;
        Ok(Self { definitions })
    }
}

impl Parse for Definition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let arrow = input.parse()?;
        let mut fields = Vec::new();

        while let Ok(field) = input.parse() {
            fields.push(field);
        }

        Ok(Self {
            name,
            arrow,
            fields,
        })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let (colon, ty) = parse_type_annotation(input)?;
        let value_variant = parse_value_variant(input)?;
        let separator = input.parse()?;
        Ok(Self {
            name,
            colon,
            ty,
            separator,
            value_variant,
        })
    }
}

fn parse_type_annotation(
    input: ParseStream,
) -> syn::Result<(Option<Token![:]>, Option<syn::Type>)> {
    if input.peek(Token![:]) {
        Ok((Some(input.parse()?), Some(input.parse()?)))
    } else {
        Ok((None, None))
    }
}

fn parse_value_variant(input: ParseStream) -> syn::Result<Option<ValueVariant>> {
    if input.peek(syn::token::Paren) {
        Ok(Some(input.parse()?))
    } else {
        Ok(None)
    }
}

impl Parse for ValueVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let paren = parenthesized!(content in input);
        let ty = content.parse()?;
        Ok(Self { paren, ty })
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
