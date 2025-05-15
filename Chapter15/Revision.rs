// use List::*;
use std::ops::Deref;
use Listz::*;
use std::rc::Rc;
fn main() {
    let n = Box::new(42);
    println!("{}", n);

    // let b = List::Cons(4,Box::new(List::Cons(5,Box::new(List::Cons(6,Box::new(List::Cons(7,Box::new(List::Nil))))))));

    let z = Boxy::new(5);
    let x = *z.deref();

    let m = Box::new(String::from("hello"));
    let k:&str = &m;
    hello(&m);

    let r = Rc::new(Listz::Cons(3,Rc::new(Listz::Cons(4,Rc::new(Listz::Nil)))));
    let p = Listz::Cons(5, Rc::clone(&r));
    let q = Listz::Cons(6, Rc::clone(&r));


}

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

enum Listz {
    Cons(i32, Rc<Listz>),
    Nil,
}

struct Boxy<T> {
    d: T,
}

impl<T> Boxy<T> {
    fn new(t : T) -> Boxy<T> {
        Boxy {d: t}
    }
}

impl<T> Deref for Boxy<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.d
    }
}

fn hello(name : &str) {
    println!("Hello, {}!", name);
}

impl<T> Drop for Boxy<T> {
    fn drop(&mut self) {
        println!("Dropping {}", self.d);
    }
}