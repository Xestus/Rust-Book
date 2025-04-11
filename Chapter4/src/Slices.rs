use std::cmp::Ordering;
use std::io;
use std::str;
use bytemuck::cast_slice;

fn main() {
    println!("Enter aa sentence: ");
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");


    let (a,b) = fi(&sentence); // immutable borrow, no more mut borrow possible
    let sen = sentence[..b].trim();
    println!("{} -> {}", sen, b);

}

fn fi(s: &str) -> (&str, usize) { // Borrows a string w/o ownership and returns a &str.
    let byte = s.as_bytes(); // string to slices
    for (i, &item) //dereference the reference to get the actual byte value.
        in byte.iter().enumerate() {
        if item == b' ' { // if byte literal for space is found, return the &str upto the usize.
            return (&s[..i], i);
        }
    }
    (&s[..], s.len()) // if not return whole string.
}

// byte.iter creates slice of bytes of the given string (&u[8]) which basically is the
// memory reference to a single character in the string and iter iterates over the number
// of bytes in the system when combined with for loop creates item as pointer to a byte.
// Reference(&x) is a pointer to memory, when dereferenced(*x), rust goes to memory it points
// and gives the value.


/*
 The inefficient version i wrote.
fn main() {
    println!("Enter a sentence: \n");
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");
    let a = meow(&sentence);
    let apple: &[u8] = cast_slice(&a);

    let peop = match str::from_utf8(apple){
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("{}", peop);

}

fn meow(s: &String) -> Vec<u8> {
    let byte = s.as_bytes();
    let mut result = Vec::new();
    let a = s.len();

    for &by in byte.iter() {
        if by == 32 {
            break;
        }
       result.push(by as u8);
    }
    result
}*/

