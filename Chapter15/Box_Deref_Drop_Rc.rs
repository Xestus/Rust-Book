// Understand page 307, 3 points.
// How does box have a known size?

use List::*;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let a = Box::new(10); // single value on heap -> inefficient
    println!("{}", *a); // Dereferencing

    // cons list -> 2 args -> to cons(truct) (a container by putting element) x onto (followed by) list y.
    // Last list -> only 1 value (Nil)

    // let l = Cons(1,Cons(2,Cons(3, Nil))); // Can't determine the size
    let l = Cons(1,Box::new(Cons(2,Box::new(Cons(3, Box::new(Nil)))))); // Can't determine the size w/o box
    // Rust doesn't care about size of variable stored in heap because heap is managed in runtime & dynamically allocated.

    let p = Bozy::new(5);
    let z = *p; // *p <=> *(p.deref())

    let mut n = Bozy::new(String::from("hello"));
    woof(&n);
}

enum List {
    Cons(i32, Box<List>),
    Nil, // Still takes up the memory in form of discriminant, like empty bottle
}

// Smart pointer has Deref(Every T value can be treated as reference)
// and Drop(All T values will be dropped after ptr gets out of scope) trait.
// Box<T> -> Store data on heap instead of stack

struct Bozy<T> {
    value : T,
}

impl<T> Bozy<T> {
    fn new(val:T) -> Self {
        Bozy { value : val }
    }
}

impl<T> Deref for Bozy<T> {
    type Target  = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

// Example of `From &T to &U when T: Deref<Target=U>`
fn exm() {
    let s = String::from("Hello");
    let s_slice: &str = &s;

}

// Example of `From &mut T to &U when T: Deref<Target=U>`
fn woof(name: &str) { // Automatic defer, as Bozy<&str> is converted to &String by fn deref
    println!("Hello, {}!", name);
}

// Example of `From &mut T to &mut U when T: DerefMut<Target=U>`
fn exam() {
    let mut v = vec![1,2,3];
    let f = &mut v[0];
}


struct SmartPointer {
    data: String,
}
impl Drop for SmartPointer {
    fn drop(&mut self) {
        println!("Dropping SmartPointer with data `{}`!", self.data);
    }
}

fn smart_pointer_solve() {
    {
        let a = SmartPointer {data: String::from("PoopAAAAAAa")};
    } // impl Drop-> fn drop auto invoked here.
    let a = SmartPointer {data: String::from("my stuff")};
    drop(a); // explicit drop not permitted (a.drop()) as rust auto calls drop causing double free error.
    println!("Stuffs");
}

enum List2 {
    Cons(i32, Rc<List>), // Hold reference -> LA -> BC can't compile
    Nil, // because &Nil will be dropped as soon it gets created.
}

fn rc_main() {
    let a = Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Nil)))));
    let b = Cons(3,Rc::clone(&a)); // Cloning the number of reference,
    let c = Cons(4,Rc::clone(&a)); // value will be dropped after it has 0 active reference.
    //Rc::clone() is preferred over a.clone() as former increases reference count, no deep copy.
    println!("{}", Rc::strong_count(&a));
}

// Multiple Owners, immutable access
// When different part use same var, no info who finishes last.