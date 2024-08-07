//create an enum to classify web events

enum  Events {
    // maybe this is like unit struct
    PageLoad,
    PageUnload,
    // like tuple struct
    KeyPress(char),
    Paste(String),
    // c-like structs
    Click{x:i64, y:i64},
    
}

enum Number {
    Zero,
    One,
    Two,
}
// explicitly assign values to each name {with explicit decriminatior}
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
    Yellow = (Color::Red as i32 | Color::Green as i32) as isize,
    White = (Color::Red as i32 | Color::Green as i32 | Color::Blue as i32) as isize,
    Black = 0x000000,
}

// define a function for the enums

fn inspect(event: Events) {
    match event {
        Events::PageLoad => println!("page loaded"),
        Events::PageUnload => println!("page unloaded"),
        Events::KeyPress(c) => println!("pressed '{}'.", c),
        Events::Paste(s) => println!("pasted \"{}\".", s),
        Events::Click{x, y} => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    println!("Hello, world!");

   
    // `to_owned()` creates an owned `String` from a string slice.

    let pressed = Events::KeyPress('x');
    inspect(pressed);

    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = Events::Paste("some text".to_owned());
    inspect(pasted);

    let clicked = Events::Click { x: 10, y: 20 };
    inspect(clicked);
    

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
    println!("sunflowers are #{:06x}", Color::Yellow as i32);
    println!("snow is #{:06x}", Color::White as i32);
    println!("night is #{:06x}", Color::Black as i32);

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
}
