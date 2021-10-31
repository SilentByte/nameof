//!
//! Rust name_of! Macro
//!
//! MIT License
//! Copyright (c) 2018 SilentByte <https://silentbyte.com/>
//!

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

    println!("{}", path_of!(crate::greet));
}
