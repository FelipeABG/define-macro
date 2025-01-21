# Grammar Definition Macro

A Rust procedural macro for defining grammar structures with a concise, declarative syntax based in the Backus-Naur Form (BNF). This macro allows you to define enums and structs using a custom grammar-like syntax that gets expanded into proper Rust code.

## Features

- Define structs and enums using a simplified grammar syntax
- Automatic capitalization of variant names for enums
- Automatic generation of constructor methods for structs
- Support for field types in parentheses
- Flexible separator syntax (comma or pipe)

## Showcase 

```rust 
define!(
    enum expr -> unary(Unary)
                |binary(Binary)
                |grouping(Grouping)
                |literal(Literal);

    struct unary -> left(Box<Expr>), operator(Token);
    struct binary -> left(Box<Expr>), operator(Token), right(Box<Expr>);
    struct grouping -> expression(Box<Expr>);
    struct literal -> value(Lit);
);
```

## Usage

The macro accepts grammar rules in the following format:

```rust
define! {
    // For structs:
    struct name -> field1(Type1), field2(Type2);

    // For enums:
    enum name -> variant1 | variant2(Type) | variant3;
}
```

### Examples

#### Defining a Struct

```rust
define! {
    struct Point -> x(f64), y(f64);
}
```

This expands to:

```rust
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
```

#### Defining an Enum

```rust
define! {
    enum Expression -> Number(i32) | Variable(String) | Plus | Minus;
}
```

This expands to:

```rust
pub enum Expression {
    Number(i32),
    Variable(String),
    Plus,
    Minus,
}
```

### Syntax Rules

1. Each rule must end with a semicolon (`;`)
2. Fields or variants can be separated by either commas (`,`) or pipes (`|`)
3. Types must be enclosed in parentheses
4. The arrow (`->`) separates the name from the fields/variants
5. Keywords `struct` or `enum` must precede the name

## Features in Detail

### Automatic Constructor Generation

For structs, the macro automatically generates a `new` method that takes all fields as parameters. This makes struct instantiation more ergonomic.

### Field Type Declarations

- For structs: `field(Type)` becomes `pub field: Type`
- For enums: `variant(Type)` becomes `Variant(Type)`
- For enums without types: `variant` becomes `Variant`

### Naming Conventions

- Struct names are kept as provided
- Enum variant names are automatically capitalized
- Field names in structs remain lowercase

## Error Handling

The macro will fail to compile with helpful error messages if:
- The grammar syntax is invalid
- Type declarations are malformed
- Required separators are missing
- Keywords are misused




