use std::rc::Rc;
use std::cell::RefCell;
use List::*;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(30)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    // value.borrow_mut() dereferences the Rc and calls .borrow_mut() on RefCell<i32>
    // converting it to RefMut<i32>
    // "*" dereferences the smart pointer.
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}


