use std::cell::RefCell;
use std::rc::Rc;
use crate::List::*;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_,  ref i) => Some(i),
            Nil => None,
        }
    }
}

fn main() {
    let p = Rc::new(Cons(10,RefCell::new(Rc::new(Cons(5,RefCell::new(Rc::new(Nil)))))));
    println!("p: {:?}", Rc::strong_count(&p));
    println!("p tail: {:?}", p.tail());

    let q = Rc::new(Cons(10, RefCell::new(Rc::clone(&p))));
    println!("q: {}", Rc::strong_count(&q));
    println!("q tail: {:?}", q.tail());

    if let Some(l) = p.tail() {
        *l.borrow_mut() = Rc::clone(&q);
    }
    // Get the tail of p (the RefCell<Rc<List>> after 10)
    // Borrow it mutably
    // Replaces its contents with an Rc pointing to q.
    // Creating a cycle where :
    // p → Cons(10, RefCell(Rc → q))
    // q → Cons(10, RefCell(Rc → p))

    println!("p tail: {:?}", p.tail());
}