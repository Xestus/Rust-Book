use std::cmp::Ordering;
use std::io;
use std::str;
use bytemuck::cast_slice;
#[derive(Debug)] // Print debugging info

/*fn main() {
    println!("Enter the height");
    let mut Height = Hold();
    println!("Enter the width");
    let mut Width = Hold();

    let u = Dimension {
        Width,
        Height,
    };

    println!("The area is {}", ValuePass(&u));
}
fn ValuePass(u : &Dimension) -> u64 {
    u.Width * u.Height
}

fn Hold() -> u64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse::<u64>().unwrap_or_else(|_| 0)
*/
/* My original was:
 match input.trim().parse::<u64>() {
        Ok(value) => value,
        Err(_) =>  0,
    } But the above is compiler recommendation
}


struct Meow {
    email: String, // String instead of str was intentional.
    active: bool,
} // No semicolon because struct definition isnt a statement, its declaration item.
// Rust likes clarity and separation of phase.
struct Moo(u32, u32);

fn initial_main() {
    let mut u1 = Meow {
        email: String::from("Hey@gmail.com"),
        active: true,
    }; // Needs semicolon because u1 is a statement
    let x = "hell";

    u1.active = false;

    let mut u2 = Meow {
        email: u1.email,
        ..u1 // if all vars below email is same as u1.
    };

    let moo1 = Moo(3, 2);

}

fn Hello(email: String, username: String) -> Meow {
    Meow {
        email,  // if = email: email,
        active: true,
    }
}
*/

/*
`String` -> Heap-allocated user-controlled string (a container with a pointer, allocated memory (capacity), currently used (length))
`str` -> String slice which stores text and its length and `[[1]]`
`&String` -> Gives you access to a string w/o taking ownership
`&str` -> A reference to string data with a length,
 actual data of `str` is unknown at compile time => Behind a fat pointer/reference (`&str`)
 `&str` = (data_ptr [8 bits], len[8 bits])

'[[1]]' `str` can live everywhere:
let x = "hello", stored in the program's binary with static lifetime (`&str` <-> `&'static str`) -> exists for the program duration.
`&String[1..5]` -> lives in heap. Sometimes in stack. Everywhere.

How movement happens:
The `&String` points to the `String` struct
Rust follows the pointer in the `String` struct to the actual string data
It creates a fat pointer `&str` using the string data address and the length from the `String`.
The capacity is left out because `str` is read-only, hence doesn't care how much memory is allocated.
*/
