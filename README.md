# nameof

[![Crate Version](https://img.shields.io/crates/v/nameof.svg)](https://crates.io/crates/nameof)
[![Build Status](https://travis-ci.org/SilentByte/nameof.svg?branch=master)](https://travis-ci.org/SilentByte/nameof)
[![MIT License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://opensource.org/licenses/MIT)

The `name_of!()` macro defined in this crate takes a binding, type, const, or
function as an argument and returns its unqualified string representation. The
`tag_of!()` macro takes an enum variant and returns its unqualified string
representation. If the identifier does not exist in the current context, the
macro will cause a compilation error. These macros are mainly intended for
debugging purposes and to improve the refactoring experience compared to
`stringify!()`.

## Usage

Add `nameof` as a dependency to your project's `Cargo.toml` file:

```toml
[dependencies]
nameof = "1.3.0"
```

To use the macro(s), import the crate with the required annotation:

```rust
use nameof::name_of;

fn main() {
    let text = "Hello, World!";
    println!("Binding `{}` holds `{}`.", name_of!(text), text);
}
```

## Examples

The `name_of!()` macro is used as follows:

```rust
use nameof::name_of;

struct TestStruct {
    test_field: i32,
}

impl TestStruct {
    const TEST_CONST: i32 = 1;
}

struct GenericStruct<T> {
    test_field_t: T,
}

fn greet() -> &'static str {
    "Hi, World"
}

fn main() {
    let text = "Hello, World!";

    println!("Binding `{}` holds `{}`.", name_of!(text), text);

    println!("Function `{}` says `{}`.", name_of!(greet), greet());

    println!(
        "Struct `{}` has a field `{}`.",
        name_of!(type TestStruct),
        name_of!(test_field in TestStruct)
    );

    println!(
        "Generic Struct `{}` has a field `{}`.",
        name_of!(type GenericStruct<String>),
        name_of!(test_field_t in GenericStruct<String>)
    );

    println!(
        "Struct `{}` has an associated constant `{}`.",
        name_of!(type TestStruct),
        name_of!(const TEST_CONST in TestStruct)
    );

    println!(
        "Standard types such as `{}` and `{}` also work.",
        name_of!(type i32),
        name_of!(type f64)
    );
}
```

Alternatively, `name_of_type!(T)` can be used instead of `name_of!(type T)`.

```rust
use nameof::name_of_type;

struct TestStruct {
    test_field: i32,
}

fn main() {
    println!("Struct is called `{}`.", name_of_type!(TestStruct));
    println!("Type is called `{}`.", name_of_type!(i32));
}
```

## Enum Variants with `tag_of!()`

The `tag_of!()` macro can be used to get the string representation of enum
variants:

```rust
use nameof::tag_of;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Hsl { h: u16, s: u8, l: u8 },
}

fn main() {
    // Unit variants
    println!("Unit variant: {}", tag_of!(Color::Red));

    // Tuple variants with range syntax - returns just variant name
    println!("Tuple variant: {}", tag_of!(Color::Rgb(..))); // "Rgb"

    // Tuple variants with specific values - returns variant name with values
    println!("Tuple variant with values: {}", tag_of!(Color::Rgb(255, 128, 0))); // "Rgb(255, 128, 0)"

    // Struct variants
    println!("Struct variant: {}", tag_of!(Color::Hsl { .. })); // "Hsl"

    // Practical usage in match expressions
    let color = Color::Rgb(255, 128, 0);
    let variant_name = match color {
        Color::Red => tag_of!(Color::Red),
        Color::Green => tag_of!(Color::Green),
        Color::Blue => tag_of!(Color::Blue),
        Color::Rgb(r, g, b) => tag_of!(Color::Rgb(*r, *g, *b)), // Returns "Rgb(255, 128, 0)"
        Color::Hsl { .. } => tag_of!(Color::Hsl { .. }),
    };
    println!("Color variant: {} -> {:?}", variant_name, color);
}
```

## License

See [LICENSE.txt](LICENSE.txt).
