use parsed::Grammar;
use proc_macro::TokenStream;
use quote::quote;

mod parsed;

/// Defines custom grammar structures using a simplified syntax.
///
/// This macro allows you to define structs and enums using a simplified grammar-like syntax.
/// The input is parsed according to the following grammar rules:
///
/// # Syntax
///
/// ```text
/// struct name -> field1(Type1), field2(Type2);
/// enum name -> variant1 | variant2(Type) | variant3;
/// ```
///
/// # Examples
///
/// ## Struct Definition
/// ```rust
/// use define_macro::define;
///
/// define! {
///     struct Point -> x(f64), y(f64);
/// }
///
/// // Expands to:
/// pub struct Point {
///      pub x: f64,
///      pub y: f64,
/// }
///
///  impl Point {
///      fn new(x: f64, y: f64) -> Self {
///          Self { x, y }
///      }
///  }
/// ```
///
/// ## Enum Definition
/// ```rust
/// use grammar_macro::define;
///
/// define! {
///     enum Expression -> Number(i32) | Variable(String) | Plus | Minus;
/// }
///
/// // Expands to:
/// // pub enum Expression {
/// //     Number(i32),
/// //     Variable(String),
/// //     Plus,
/// //     Minus,
/// // }
/// ```
#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as Grammar);
    quote! {#ast}.into()
}
