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
    pub arrow: Token![->],
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: syn::Ident,
    pub paren: Option<syn::token::Paren>,
    pub ty: Option<syn::Type>,
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
            arrow,
            fields,
        })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let (paren, ty) = parse_type(input)?;
        parse_separator(input)?;
        Ok(Self { name, paren, ty })
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
