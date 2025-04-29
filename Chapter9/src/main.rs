use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Error;

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; v[999];

    let f2= File::open("moo.txt");

    let x = match f2{
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("moo.txt") {
                Ok(f) => f,
                Err(e) => {
                    panic!("Error opening file: {:?}", e)
                    // ";" is optional because panic!() will never return as it causes program to crash.
                } // "," is optional because there is nothing after it.
            }
        },
        // Err(e)=> return Err(e), // Doesn't work but error_help works
        Err(e) => { panic!("Error opening file: {:?}", e) } //placeholder
    };


    // Shorter version:
    let f3 = File::open("moo.txt").unwrap();
    let f4 = File::open("moo.txt").expect("Unable to open file");
}

fn error_help() -> Result<String, Error>
{
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // Ends fn because we're requesting a return of Result<String, Error>
    };
    // The below does nothing different but it can be compressed into "?" operator.
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

/*    let mut s = String::new();

  // Check Result value, return if Err, continue if Ok
  let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    File::open("hello.txt")?.read_to_string(&mut s)?;

    "?" can only be used if a function is returning Result<T, E>
*/
}

/*
    Err(&error) cannot be written because the error in result stands for std::io::Error,
     the actual `Error` value, not the reference.
 */


loop {
let guess: i32 = match guess.trim().parse() {
Ok(num) => num,
Err(_) => continue,
};
if guess < 1 || guess > 100 {
println!("The secret number will be between 1 and 100.");
continue;
}
match guess.cmp(&secret_number) {
// --snip--
}

pub struct Guess {
value: u32,
}
impl Guess {
pub fn new(value: u32) -> Guess {
if value < 1 || value > 100 {
panic!("Guess value must be between 1 and 100, got {}.", value);
}
 Guess {
value
}
}
pub fn value(&self) -> u32 {
self.value
}
}