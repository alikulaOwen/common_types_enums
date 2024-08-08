//create an enum to classify web events

use crate::List::*;
use std::convert::{From, Into, TryFrom};
use std::fmt;

struct Circle{
    radius: i32
}

impl fmt::Display for Circle{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cirle of radius{}", self.radius)
    }
}


#[derive(Debug)]
struct Num{
    value: i32,
}
impl From<i32> for Num{
    fn from(item: i32)-> Self{
        Num{value: item}
    }
}

struct Even{
    value: i32
}

impl TryFrom<i32> for Even{
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0{
            Ok(Even{value})

        }else {
            Err("Number is not Even")
        }
    }

}




enum List{
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}


impl List{
    //c reats new empty list
    fn new()-> List {
        Nil
    }

    fn prepend(self, elem: u32)-> List{
        Cons(elem, Box::new(self))
    }

    fn len(&self)-> u32{
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // base case: an empty list has zero length
            Nil => 0
        }
    }
    fn stringify(&self)-> String{
        match *self{
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}

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

static LANGUAGE: &str = "PYTHON";
const THRESHOLD: i32 = 10;








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

fn is_big(n: i32) ->bool {
    n > THRESHOLD
}

type Second = u64;
type Distance = u64;
fn main() {
    println!("Hello, world!");
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(4);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
   
    // `to_owned()` creates an owned `String` from a string slice.

    let pressed = Events::KeyPress('x');
    inspect(pressed);

    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = Events::Paste("some text".to_owned());
    inspect(pasted);

    let clicked = Events::Click { x: 10, y: 20 };
    inspect(clicked);
    let n  = 16;

    print!("{} ", n);
    println!("{} ", n);
    println!("{} ", n);
    println!("{} ", THRESHOLD);
    println!("{} ", LANGUAGE);
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});


    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
    println!("sunflowers are #{:06x}", Color::Yellow as i32);
    println!("snow is #{:06x}", Color::White as i32);
    println!("night is #{:06x}", Color::Black as i32);

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);


    //  `variable binding ` mutablilty

    let _immute = 1;
    let mut mutable = 1;
    println!("Before mutation: {}", mutable);

    mutable += 1;

    println!("After mutation : {}", mutable );
    mutable += 1;

    println!("After mutation 2 : {}", mutable);  

    // `type conversions`

    // rust has limited support for type coercion
    // typical occurs in contexts like `Coverting fromn concrete type to a trait object ` and `Auto deferencing and deref coerciomn (ie converting &String to &str)`

    let s:  &str = "Hello";
    let string: String = String::from(s);

    println!("{} world!", string); 
    // `Type casting`

    // this is where a programmer explicit define syntax in code for type conversion
    // in rust we use `as` keyword

    let x: i32 = 5;

    let _y: u32 = x as u32;

    //LITERALS
    // numerical literals can be type annonated by adding type suffix
    // ie 42 can be 132 by wring 42i32

    let _x = 42i32;

    //INferrence
    let element = 5u8;
    // creating an empty vector(growable array)
    let mut vec = Vec::new();


    vec.push(element);

    println!("{:?}", vec);

    //ALIASING
    // type staements can be used to give a new name to exsting type.
    // must have UpperCamelCase or compile willraise warning
    // exceptions are only primitive types
    let seconds: Second = 10 as u64;
    let metres: Distance = 200 as u64;

    println!("{} seconds and {} metres", seconds, metres);

     //from `CONVERSION`
     let numbers = Num::from(35);
     let num: Num = 45.into();
     println!("My number is {:?}", numbers.value);
     println!("My number into {:?}", num.value);

     //TryFrom

     let num = 42;
      let even_number = Even::try_from(num);

      match even_number{
        Ok(n)=> println!("Converted to Even: {:?}", n.value),
        Err(e)=> println!("Failed to convert: {}", e),
      }

      // TryFrom

    // assert_eq!(Even::try_from(8), Ok(Even(8)));
    // assert_eq!(Even::try_from(5), Err(()));

    // // TryInto

    // let result: Result<Even, ()> = 8i32.try_into();
    // assert_eq!(result, Ok(Even(8)));
    // let result: Result<Even, ()> = 5i32.try_into();
    // assert_eq!(result, Err(()));

    // // TryFrom
    // assert_eq!(Even::try_from(8), Ok(Even(8)));
    // assert_eq!(Even::try_from(5), Err("Value is not even"));

    // // TryInto
    // let result: Result<Even, _> = 8i32.try_into();
    // assert_eq!(result, Ok(Even(8)));
    // let result: Result<Even, _> = 5i32.try_into();
    // assert_eq!(result, Err("Value is not even"));
    

    let circle = Circle{ radius: 6};
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
