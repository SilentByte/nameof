//!
//! Rust name_of! Macro
//!
//! MIT License
//! Copyright (c) 2018 SilentByte <https://silentbyte.com/>
//!

#![crate_name = "nameof"]
#![no_std]

/// Takes a binding, type, const, or function as an argument and returns its
/// unqualified string representation. If the identifier does not exist
/// in the current context, the macro will cause a compilation error.
/// This macro is mainly intended for debugging purposes and to improve
/// the refactoring experience compared to plain `stringify!()`.
///
/// The syntax depends on the type of the identifier:
///
/// 1. Bindings to variables and functions require no annotation,
///    e.g. `name_of!(some_binding)`.
///
/// 2. Types and structs require the keyword `type`, e.g. `name_of!(type SomeType)`.
///    Alternatively, the macro `name_of_type!(SomeType)` may be used.
///
/// 3. Fields within structs are referred to with the `in` keyword,
///    e.g. `name_of!(some_field in SomeType)`.
///
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate nameof;
/// # fn main() {
/// struct TestStruct {
///     test_field: i32,
/// }
///
/// impl TestStruct {
///     const TEST_CONST: i32 = 1;
/// }
///
/// struct GenericStruct<T> {
///     test_field_t: T,
/// }
///
/// fn greet() -> &'static str {
///     "Hi, World"
/// }
///
/// let text = "Hello, World!";
///
/// println!("Binding `{}` holds `{}`.", name_of!(text), text);
///
/// println!("Function `{}` says `{}`.", name_of!(greet), greet());
///
/// println!(
///     "Struct `{}` has a field `{}`.",
///     name_of!(type TestStruct),
///     name_of!(test_field in TestStruct)
/// );
///
/// println!(
///     "Generic Struct `{}` has a field `{}`.",
///     name_of!(type GenericStruct<String>),
///     name_of!(test_field_t in GenericStruct<String>)
/// );
///
/// println!(
///     "Struct `{}` has an associated constant `{}`.",
///     name_of!(type TestStruct),
///     name_of!(const TEST_CONST in TestStruct)
/// );
///
/// println!(
///     "Standard types such as `{}` and `{}` also work.",
///     name_of!(type i32),
///     name_of!(type f64)
/// );
///
/// # }
/// ```
#[macro_export]
macro_rules! name_of {
    // Covers Bindings
    ($n: ident) => {{
        let _ = || {
            let _ = &$n;
        };
        stringify!($n)
    }};

    // Covers Types
    (type $t: ty) => {{
        $crate::name_of_type!($t)
    }};

    // Covers Struct Fields
    ($n: ident in $t: ty) => {{
        let _ = |f: $t| {
            let _ = &f.$n;
        };
        stringify!($n)
    }};

    // Covers Struct Constants
    (const $n: ident in $t: ty) => {{
        let _ = || {
            let _ = &<$t>::$n;
        };
        stringify!($n)
    }};
}

/// Takes the name of a type as its sole parameter,
/// e.g. `name_of_type!(SomeStruct)` or `name_of_type!(f64)`.
///
/// It is an alternative to the `name_of!(type T)` macro, specifically for types.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate nameof;
/// # fn main() {
/// struct TestStruct {
///     test_field: i32
/// }
///
/// println!("Struct is called `{}`.", name_of_type!(TestStruct));
/// println!("Type is called `{}`.", name_of_type!(i32));
///
/// # }
/// ```
#[macro_export]
macro_rules! name_of_type {
    // Covers Types
    ($t: ty) => {{
        let _ = || {
            let _: $t;
        };
        stringify!($t)
    }};
}

/// Takes an enum variant as its parameter and returns its unqualified string representation.
/// If the enum variant does not exist in the current context, the macro will cause a compilation error.
/// This macro is mainly intended for debugging purposes and to improve the refactoring experience.
///
/// The syntax supports different forms of enum variants with different output formats:
///
/// 1. Unit variants: `tag_of!(SomeEnum::Variant)` → `"Variant"`
/// 2. Tuple variants with range: `tag_of!(SomeEnum::Variant(..))` → `"Variant"`
/// 3. Tuple variants with values: `tag_of!(SomeEnum::Variant(value1, value2))` → `"Variant(value1, value2)"`
/// 4. Struct variants: `tag_of!(SomeEnum::Variant { .. })` → `"Variant"`
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate nameof;
/// # fn main() {
/// #[derive(Debug)]
/// enum Color {
///     Red,
///     Green,
///     Blue,
///     Rgb(u8, u8, u8),
///     Hsl { h: u16, s: u8, l: u8 },
/// }
///
/// println!("Unit variant: {}", tag_of!(Color::Red)); // "Red"
/// println!("Tuple variant: {}", tag_of!(Color::Rgb(..))); // "Rgb"
/// println!("Tuple variant with values: {}", tag_of!(Color::Rgb(255, 128, 0))); // "Rgb(255, 128, 0)"
/// println!("Struct variant: {}", tag_of!(Color::Hsl { .. })); // "Hsl"
/// # }
/// ```
#[macro_export]
macro_rules! tag_of {
    // Unit variants: EnumName::Variant
    ($enum_name:ident :: $variant:ident) => {{
        let _ = || {
            let _ = $enum_name::$variant;
        };
        stringify!($variant)
    }};

    // Tuple variants with range: EnumName::Variant(..)
    ($enum_name:ident :: $variant:ident ( .. )) => {{
        let _ = || {
            // Use pattern matching to verify the variant exists
            match None::<$enum_name> {
                Some($enum_name::$variant(..)) => {}
                _ => {}
            }
        };
        stringify!($variant)
    }};

    // Tuple variants with specific values: EnumName::Variant(value1, value2, ...)
    ($enum_name:ident :: $variant:ident ( $($value:expr),+ )) => {{
        let _ = || {
            let _ = $enum_name::$variant($($value,)*);
        };
        {
            extern crate std;
            let variant_name = stringify!($variant);
            let values = std::vec![$(std::format!("{:?}", $value)),+];
            std::format!("{}({})", variant_name, values.join(", "))
        }
    }};

    // Struct variants with fields: EnumName::Variant { .. }
    ($enum_name:ident :: $variant:ident { .. }) => {{
        let _ = || {
            // Use pattern matching to verify the variant exists
            match None::<$enum_name> {
                Some($enum_name::$variant { .. }) => {}
                _ => {}
            }
        };
        stringify!($variant)
    }};
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {

    use std::string::{String, ToString};

    fn test_fn() {
        //
    }

    struct TestStruct {
        test_field: i32,
    }

    impl TestStruct {
        const TEST_CONST: i32 = 1;
    }

    struct TestGenericStruct<T> {
        test_field: T,
    }

    struct TestGenericStructMultiType<T, U> {
        test_field_t: T,
        test_field_u: U,
    }

    #[derive(Debug)]
    enum TestEnum {
        UnitVariant,
        TupleVariant(i32),
        TupleVariantMultiple(i32, String),
        StructVariant { field1: i32, field2: String },
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
    fn name_of_type() {
        assert_eq!(name_of!(type i32), "i32");
        assert_eq!(name_of_type!(i32), "i32");
    }

    #[test]
    fn name_of_struct() {
        assert_eq!(name_of!(type TestStruct), "TestStruct");
        assert_eq!(name_of_type!(TestStruct), "TestStruct");
    }

    #[test]
    fn name_of_generic_struct() {
        assert_eq!(
            name_of!(type TestGenericStruct<i32>),
            "TestGenericStruct<i32>"
        );

        assert_eq!(
            name_of_type!(TestGenericStruct<i32>),
            "TestGenericStruct<i32>"
        );
    }

    #[test]
    fn name_of_generic_multi_type_struct() {
        assert_eq!(
            name_of!(type TestGenericStructMultiType<i32, TestGenericStruct<String>>),
            "TestGenericStructMultiType<i32, TestGenericStruct<String>>"
        );

        assert_eq!(
            name_of_type!( TestGenericStructMultiType<i32, TestGenericStruct<String>>),
            "TestGenericStructMultiType<i32, TestGenericStruct<String>>"
        );
    }

    #[test]
    fn name_of_struct_field() {
        assert_eq!(name_of!(test_field in TestStruct), "test_field");
    }

    #[test]
    fn name_of_generic_struct_field() {
        assert_eq!(name_of!(test_field in TestGenericStruct<i32>), "test_field");
    }

    #[test]
    fn name_of_generic_multi_type_struct_field() {
        assert_eq!(
            name_of!(test_field_t in TestGenericStructMultiType<i32, TestGenericStruct<String>>),
            "test_field_t"
        );

        assert_eq!(
            name_of!(test_field_u in TestGenericStructMultiType<i32, TestGenericStruct<String>>),
            "test_field_u"
        );
    }

    #[test]
    fn name_of_struct_constant() {
        assert_eq!(name_of!(const TEST_CONST in TestStruct), "TEST_CONST");
    }

    #[test]
    fn tag_of_unit_variant() {
        assert_eq!(tag_of!(TestEnum::UnitVariant), "UnitVariant");
    }

    #[test]
    fn tag_of_tuple_variant() {
        assert_eq!(tag_of!(TestEnum::TupleVariant(..)), "TupleVariant");
    }

    #[test]
    fn tag_of_tuple_variant_multiple() {
        assert_eq!(
            tag_of!(TestEnum::TupleVariantMultiple(..)),
            "TupleVariantMultiple"
        );
    }

    #[test]
    fn tag_of_struct_variant() {
        assert_eq!(tag_of!(TestEnum::StructVariant { .. }), "StructVariant");
    }

    #[test]
    fn tag_of_tuple_variant_with_values() {
        assert_eq!(tag_of!(TestEnum::TupleVariant(42)), "TupleVariant(42)");
    }

    #[test]
    fn tag_of_tuple_variant_multiple_with_values() {
        assert_eq!(
            tag_of!(TestEnum::TupleVariantMultiple(42, "test".to_string())),
            "TupleVariantMultiple(42, \"test\")"
        );
    }
}
