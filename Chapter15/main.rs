use std::cell::RefCell;
use std::rc::*;
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let l = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()), // A dangling weak pointer
        children: RefCell::new(vec![]),
    }); // wrapped around Rc -> Shared ownership
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&l),
        Rc::weak_count(&l),
    );

    // println!("parent = {:?}", l.parent.borrow().upgrade());
    // immutable borrow from RefCell, .upgrade() returns none as its dangling pointer.

    {
        let m = Rc::new(Node {
            value: 10,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&l)]), // points to earlier node
            // Creates m -> l (strong pointer)
        });
        *l.parent.borrow_mut() = Rc::downgrade(&m);
        // mut borrowing l.parent and assigning a weak pointer of m.
        // l -> m (weak pointer)

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&l),
            Rc::weak_count(&l),
        );

        println!(
            "Branch strong = {}, weak = {}",
            Rc::strong_count(&m),
            Rc::weak_count(&m),
        );

    }

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&l),
        Rc::weak_count(&l),
    );
    // println!("parent = {:?}", l.parent.borrow().upgrade());
    // Print Some value because the weak reference can be upgraded to a strong one
}
