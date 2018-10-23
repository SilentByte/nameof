
nameof
======
[![Crate Version](https://img.shields.io/crates/v/nameof.svg)](https://crates.io/crates/nameof)
[![Build Status](https://travis-ci.org/SilentByte/nameof.svg?branch=master)](https://travis-ci.org/SilentByte/nameof)
[![MIT License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://opensource.org/licenses/MIT)

The `name_of!()` macro defined in this crate takes a binding, type, or function as an argument and returns its unqualified string representation. If the identifier does not exist in the current context, the macro will cause a compilation error. This macro is mainly intended for debugging purposes and to improve the refactoring experience compared to `stringify!()`.


## Usage

Add `nameof` as a dependency to your project's `Cargo.toml` file:

```toml
[dependencies]
nameof = "1.0"
```

To use the macro(s), import the crate with the required annotation:

```rust
#[macro_use]
extern crate nameof;

fn main() {
    let text = "Hello, World!";
    println!("Binding `{}` holds `{}`.", name_of!(text), text);
}
```


## Examples

The `name_of!()` macro is used as follows:

```rust
#[macro_use]
extern crate nameof;

struct TestStruct {
    test_field: i32,
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
        "Standard types such as `{}` and `{}` also work.",
        name_of!(type i32),
        name_of!(type f64)
    );
}
```

Alternatively, `name_of_type!(T)` can be used instead of `name_of!(type T)`.

```rust
#[macro_use]
extern crate nameof;

struct TestStruct {
    test_field: i32,
}

fn main() {
    println!("Struct is called `{}`.", name_of_type!(TestStruct));
    println!("Type is called `{}`.", name_of_type!(i32));
}
```

## License

See [LICENSE.txt](LICENSE.txt).

