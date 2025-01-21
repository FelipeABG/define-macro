use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
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

impl ToTokens for Grammar {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let rules: Vec<&Rule> = self.rules.iter().collect();

        let ts = quote! {
            #(#rules)*
        };

        tokens.extend(ts);
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

impl ToTokens for Rule {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = syn::Ident::new(
            &format!("{}", capitalize(&self.name.to_string())),
            self.name.span(),
        );
        let fields = &self.fields;

        let ts = match &self.fields[0].separator {
            Separator::Comma => {
                quote! {
                    struct #name {
                        #(#fields)*
                    }
                }
            }
            Separator::Pipe => {
                quote! {
                    enum #name {
                        #(#fields)*
                    }
                }
            }
        };

        tokens.extend(ts);
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

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ts = match self.separator {
            Separator::Comma => {
                let name = &self.name;
                let ty = &self.ty.clone().unwrap();
                quote! {#name: #ty,}
            }
            Separator::Pipe => {
                let name = &syn::Ident::new(
                    &format!("{}", capitalize(&self.name.to_string())),
                    self.name.span(),
                );

                match &self.ty {
                    Some(ty) => {
                        quote! {#name(#ty),}
                    }
                    None => quote! {#name,},
                }
            }
        };

        tokens.extend(ts);
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
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
            Err(input.error("Expected '|' or ','"))
        }
    }
}
