
struct Ptr {
    x: i32,
    y: i32,
    z: i32,
    p: i32,
    q: i32,
    r: i32,
}

fn main() {
    let mut a = Err("Hi");
    let b = Ok(10);
    match (a, b) {
        (Err(_), Err(_)) | (Ok(_), Ok(_)) => {
            println!("Error, can't modify the variable");
        }
        _ => {
            a = b;
        }
    }

    let num = (2, 4, 6, 8, 10);
    match num {
        (1, _, 2, _, _) => { println!("{} {}", 1, 2) },
        (1, .., 3) => println!("{}", 1),
        _ => println!("No hiHi")
    }

    let _q = 1; // avoids unused variable warning
    println!("{}", _q);

    let ptr = Ptr { x: 4, y: 6, z: 10, p: 10, q: 10, r: 10 };

    match ptr {
        (Ptr { p, .. }) => println!("ptr is of type"),
        _ => println!("ptr is not of type")
    }

    // As `&` already have a meaning in patterns, it can't be used to create reference in patterns

    let mut r = Some(String::from("hello"));
    let s = String::from("caw caw");
    let c = 10;

    match r {
        Some(ref mut k) if k.len() > 3 => {*k = String::from("Poopa")},
        Some(ref k) | Some(s) if k.len() == c => println!("{}",k),
        // the if k.len() == c applies to both the some.
        None => println!("None"),
        _ => println!("no such thing"),
    };
    println!("{:?}", r);

    

}
fn foo(_:i32,y:i32) -> i32 {
    println!("{}", y);
    y
}