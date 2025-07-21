//! Example demonstrating the usage of name_of! macro with enum variants.

#[macro_use]
extern crate nameof;

#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Hsl { h: u16, s: u8, l: u8 },
}

#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("=== name_of! macro with enum variants ===\n");

    println!("Unit variants:");
    println!("  {} -> '{}'", "name_of!(Color::Red)", name_of!(Color::Red));
    println!(
        "  {} -> '{}'",
        "name_of!(Color::Green)",
        name_of!(Color::Green)
    );
    println!(
        "  {} -> '{}'",
        "name_of!(Message::Quit)",
        name_of!(Message::Quit)
    );
    println!();

    println!("Tuple variants with range syntax (..):");
    println!(
        "  {} -> '{}'",
        "name_of!(Color::Rgb(..))",
        name_of!(Color::Rgb(..))
    );
    println!(
        "  {} -> '{}'",
        "name_of!(Message::Write(..))",
        name_of!(Message::Write(..))
    );
    println!(
        "  {} -> '{}'",
        "name_of!(Message::ChangeColor(..))",
        name_of!(Message::ChangeColor(..))
    );
    println!();

    println!("Tuple variants with specific values:");
    println!(
        "  {} -> '{}'",
        "name_of!(Color::Rgb(255, 128, 0))",
        name_of!(Color::Rgb(255, 128, 0))
    );
    println!(
        "  {} -> '{}'",
        "name_of!(Color::Rgb(0, 0, 0))",
        name_of!(Color::Rgb(0, 0, 0))
    );
    println!(
        "  {} -> '{}'",
        "name_of!(Message::Write(\"hello\".to_string()))",
        name_of!(Message::Write("hello".to_string()))
    );
    println!(
        "  {} -> '{}'",
        "name_of!(Message::ChangeColor(255, 255, 255))",
        name_of!(Message::ChangeColor(255, 255, 255))
    );
    println!();

    println!("Struct variants:");
    println!(
        "  {} -> '{}'",
        "name_of!(Color::Hsl {{ .. }})",
        name_of!(Color::Hsl { .. })
    );
    println!(
        "  {} -> '{}'",
        "name_of!(Message::Move {{ .. }})",
        name_of!(Message::Move { .. })
    );
    println!();

    println!("Unit variant:");
    println!("  name_of!(Color::Red) -> '{}'", name_of!(Color::Red));
    println!();

    println!("Range syntax:");
    println!(
        "  name_of!(Color::Rgb(..)) -> '{}'",
        name_of!(Color::Rgb(..))
    );

    println!();

    println!("Specific values:");
    println!(
        "  name_of!(Color::Rgb(255, 0, 128)) -> '{}'",
        name_of!(Color::Rgb(255, 0, 128))
    );

    println!();

    let colors = vec![
        Color::Red,
        Color::Rgb(255, 128, 0),
        Color::Hsl {
            h: 240,
            s: 100,
            l: 50,
        },
    ];

    for color in &colors {
        match color {
            Color::Red => println!("  Processing: {} -> {:?}", name_of!(Color::Red), color),
            Color::Green => println!("  Processing: {} -> {:?}", name_of!(Color::Green), color),
            Color::Blue => println!("  Processing: {} -> {:?}", name_of!(Color::Blue), color),
            Color::Rgb(r, g, b) => println!(
                "  Processing: {} -> {:?}",
                name_of!(Color::Rgb(*r, *g, *b)),
                color
            ),
            Color::Hsl { .. } => println!(
                "  Processing: {} -> {:?}",
                name_of!(Color::Hsl { .. }),
                color
            ),
        }
    }

    struct TestStruct {
        test_field: i32,
    }

    impl TestStruct {
        const TEST_CONST: i32 = 1;
    }

    let text = "Hello, World!";

    println!("Binding: {}", name_of!(text));
    println!("Type: {}", name_of!(type TestStruct));
    println!("Field: {}", name_of!(test_field in TestStruct));
    println!("Constant: {}", name_of!(const TEST_CONST in TestStruct));

    println!("Enum variant: {}", name_of!(Color::Red));
    println!(
        "Enum variant with values: {}",
        name_of!(Color::Rgb(255, 255, 255))
    );
}
