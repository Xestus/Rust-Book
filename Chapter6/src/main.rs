#[derive(Debug)]


/*enum IP {
    V4(u8,u8,u8,u8), // let x = IP::V4(127,0,0,1)
    V6(IPv6),
}
struct IPv6 {}*/

enum Message {
    Write(String), // We can create a struct of all of them
}

impl Message {
    fn meow(&self) {
        // println!("Message is {}", self);
        // Rust won't automatically unpack its value even though there is only 1 variant.
        println!("Message is {:?}", self);
    }
}

fn main() {
    let woof = Message::Write(String::from("hello"));
    woof.meow();

    let a = Some(5); // a is Schrödinger's datatype. Can be anything till you force a concrete datatype onto it.
    let b = add111(a);
    let num = add111(None);

}

enum Koin { Dime(Color), }
#[derive(Debug)]
enum Color { Red, }

fn value(coin: Koin) -> u32{
    match coin {
        Koin::Dime(color) => {
            println!("Hi {:?}", color);
            10
        },
        // unreachable pattern because the value can only take Koin enum but Koin only has 1 variable inside it and we've covered it.
        // _ => 0,
    }
}

fn add111(a : Option<u32>) -> Option<u32> {
    match a {
        None => None,
        Some(i) => Some(i + i),
    }
}

fn add222(a : Option<u32>) -> Option<u32> {
    if let Some(i) = a {
        Some(i + i)
    }
    else {
        None
    }
}

// Match is preferred over If because match forces you to handle every condition
// and its easier to spot Option<T> conditions using match

// If let can be used when doing one variant solution and single line filtering.