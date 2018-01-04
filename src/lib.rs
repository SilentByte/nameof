//!
//! Rust name_of! Macro
//!
//! MIT License
//! Copyright (c) 2018 SilentByte <https://silentbyte.com/>
//!

/// Takes a variable, type, or function as an argument and returns its
/// unqualified string representation. If the identifier does not exist
/// in the current context, the macro will cause a compilation error.
/// This macro is mainly intended for debugging purposes and to improve
/// the refactoring experience compared to `stringify!()`.
#[macro_export]
macro_rules! name_of {
    // Covers Bindings
    ($n: ident) => {{
        || { &$n; };
        stringify!($n)
    }};

    // Covers Structs
    (type $t: ty) => {{
        || { let _: $t; };
        stringify!($t)
    }};

    // Covers Struct Fields
    ($n: ident for $t: ty) => {{
        |f: $t| { let _ = &f.$n; };
        stringify!($n)
    }};
}

#[cfg(test)]
mod tests {
    fn test_fn() {
        //
    }

    struct TestStruct {
        test_field: i32,
    }

    struct TestGenericStruct<T> {
        test_field: T,
    }

    #[test]
    fn name_of_binding() {
        let test_variable = 123;
        assert_eq!(name_of!(test_variable), "test_variable");
    }

    #[test]
    fn name_of_fn() {
        assert_eq!(name_of!(test_fn), "test_fn");
    }

    #[test]
    fn name_of_struct() {
        assert_eq!(name_of!(type TestStruct), "TestStruct");
    }

    #[test]
    fn name_of_generic_struct() {
        assert_eq!(
            name_of!(type TestGenericStruct<i32>),
            "TestGenericStruct<i32>"
        );
    }

    #[test]
    fn name_of_struct_field() {
        assert_eq!(name_of!(test_field for TestStruct), "test_field");
    }

    #[test]
    fn name_of_generic_struct_field() {
        assert_eq!(
            name_of!(test_field for TestGenericStruct<i32>),
            "test_field"
        );
    }
}
