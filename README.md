
nameof
======
[![Build Status](https://travis-ci.org/SilentByte/nameof.svg?branch=master)](https://travis-ci.org/SilentByte/nameof)
[![MIT License](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://opensource.org/licenses/MIT)

This Rust crate provides a macro to determine the string name of an identifier or type.

## Usage
/// Takes a variable, type, or function as an argument and returns its
/// unqualified string representation. If the identifier does not exist
/// in the current context, the macro will cause a compilation error.
/// This macro is mainly intended for debugging purposes and to improve
/// the refactoring experience compared to `stringify!()`.
///
/// # Examples
///
/// ```
/// // Bindings
/// let text = "Hello, World!";
/// println!("Variable `{}` holds `{}`.", name_of!(text), text);
///
/// // Functions
/// fn greet() { }
/// println!("Function is called `{}`.", name_of!(greet));
///
/// // Types & Fields
/// struct TestStruct { test_field: i32 }
/// println!("Struct is called `{}`.", name_of!(type TestStruct));
/// println!("Field is called `{}`.", name_of!(test_field for TestStruct));
/// println!("Standard Types: `{}`.", name_of!(i32));
