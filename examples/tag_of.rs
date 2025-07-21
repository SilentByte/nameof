//! Comprehensive example demonstrating the usage of the tag_of! macro for enum variants.

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
    println!("=== tag_of! macro comprehensive examples ===\n");

    // Unit variants - always return just the variant name
    println!("Unit variants:");
    println!("  {} -> '{}'", "Color::Red", tag_of!(Color::Red));
    println!("  {} -> '{}'", "Color::Green", tag_of!(Color::Green));
    println!("  {} -> '{}'", "Message::Quit", tag_of!(Message::Quit));
    println!();

    // Tuple variants with range syntax - return just the variant name
    println!("Tuple variants with range syntax (..):");
    println!("  {} -> '{}'", "Color::Rgb(..)", tag_of!(Color::Rgb(..)));
    println!(
        "  {} -> '{}'",
        "Message::Write(..)",
        tag_of!(Message::Write(..))
    );
    println!(
        "  {} -> '{}'",
        "Message::ChangeColor(..)",
        tag_of!(Message::ChangeColor(..))
    );
    println!();

    // Tuple variants with specific values - return variant name with values
    println!("Tuple variants with specific values:");
    println!(
        "  {} -> '{}'",
        "Color::Rgb(255, 128, 0)",
        tag_of!(Color::Rgb(255, 128, 0))
    );
    println!(
        "  {} -> '{}'",
        "Color::Rgb(0, 0, 0)",
        tag_of!(Color::Rgb(0, 0, 0))
    );
    println!(
        "  {} -> '{}'",
        "Message::Write(\"hello\".to_string())",
        tag_of!(Message::Write("hello".to_string()))
    );
    println!(
        "  {} -> '{}'",
        "Message::ChangeColor(255, 255, 255)",
        tag_of!(Message::ChangeColor(255, 255, 255))
    );
    println!();

    // Struct variants - return just the variant name
    println!("Struct variants:");
    println!(
        "  {} -> '{}'",
        "Color::Hsl {{ .. }}",
        tag_of!(Color::Hsl { .. })
    );
    println!(
        "  {} -> '{}'",
        "Message::Move {{ .. }}",
        tag_of!(Message::Move { .. })
    );
    println!();

    // Practical usage examples
    println!("=== Practical usage examples ===");

    // Example 1: Using in match expressions
    println!("\n1. Using in match expressions:");
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
        let variant_name = match color {
            Color::Red => tag_of!(Color::Red),
            Color::Green => tag_of!(Color::Green),
            Color::Blue => tag_of!(Color::Blue),
            Color::Rgb(..) => tag_of!(Color::Rgb(..)), // Just variant name
            Color::Hsl { .. } => tag_of!(Color::Hsl { .. }),
        };
        println!("  Color variant: {} -> {:?}", variant_name, color);
    }

    // Example 2: Getting detailed variant info with values
    println!("\n2. Getting detailed variant info with values:");
    for color in &colors {
        match color {
            Color::Red => println!("  Detailed: {} -> {:?}", tag_of!(Color::Red), color),
            Color::Green => println!("  Detailed: {} -> {:?}", tag_of!(Color::Green), color),
            Color::Blue => println!("  Detailed: {} -> {:?}", tag_of!(Color::Blue), color),
            Color::Rgb(r, g, b) => println!(
                "  Detailed: {} -> {:?}",
                tag_of!(Color::Rgb(*r, *g, *b)),
                color
            ),
            Color::Hsl { .. } => {
                println!("  Detailed: {} -> {:?}", tag_of!(Color::Hsl { .. }), color)
            }
        }
    }

    // Example 3: Debugging and logging
    println!("\n3. Debugging and logging scenarios:");
    let message = Message::ChangeColor(255, 0, 128);
    match message {
        Message::ChangeColor(r, g, b) => {
            println!("  Processing: {}", tag_of!(Message::ChangeColor(r, g, b)));
            println!("  RGB values: r={}, g={}, b={}", r, g, b);
        }
        _ => {}
    }

    // Example 4: Comparison of different syntaxes
    println!("\n4. Comparison of different syntaxes:");
    println!("  Range syntax:    {}", tag_of!(Color::Rgb(..))); // "Rgb"
    println!("  Specific values: {}", tag_of!(Color::Rgb(255, 128, 0))); // "Rgb(255, 128, 0)"
    println!("  Unit variant:    {}", tag_of!(Color::Red)); // "Red"
    println!("  Struct variant:  {}", tag_of!(Color::Hsl { .. })); // "Hsl"
}
