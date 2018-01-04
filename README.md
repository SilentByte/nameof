
nameof
======
[![Build Status](https://travis-ci.org/SilentByte/nameof.svg?branch=master)](https://travis-ci.org/SilentByte/nameof)
[![MIT License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://opensource.org/licenses/MIT)

The `name_of!()` macro defined in this crate takes a binding, type, or function as an argument and returns its unqualified string representation. If the identifier does not exist in the current context, the macro will cause a compilation error. This macro is mainly intended for debugging purposes and to improve the refactoring experience compared to `stringify!()`.


## Usage

Add `nameof` as a dependency to your project's `Cargo.toml` file:

```toml
[dependencies]
nameof = "*"
```

To use the macro(s), import the crate with the required annotation:

```rust
#[macro_use]
extern crate nameof;

fn main() {
    let text = "Hello, World!";
    println!("Variable `{}` holds `{}`.", name_of!(text), text);
}
```


## Examples

The `name_of!()` macro is used as follows:

```rust
#[macro_use]
extern crate nameof;

fn main() {
    // Bindings
    let text = "Hello, World!";
    println!("Variable `{}` holds `{}`.", name_of!(text), text);
    
    // Functions
    fn greet() { }
    println!("Function is called `{}`.", name_of!(greet));
    
    // Types & Fields
    struct TestStruct { test_field: i32 }
    println!("Struct is called `{}`.", name_of!(type TestStruct));
    println!("Standard Types: `{}`.", name_of!(type i32));
    println!("Field is called `{}`.", name_of!(test_field for TestStruct));
}
```

Alternatively, `name_of_type!(T)` can be used instead of `name_of!(type T)`.

```rust
#[macro_use]
extern crate nameof;

fn main() {
    struct TestStruct { test_field: i32 }
    println!("Struct is called `{}`.", name_of_type!(TestStruct));
    println!("Struct is called `{}`.", name_of_type!(i32));
}
```

## License

See [LICENSE.txt](LICENSE.txt).

