fn main() {
    let x = 4;
    let eql = move |z:i32| -> bool {
        z == x // x doesn't exist inside the closure but takes params from scope.
        // Ownership, immut borrow & mut borrow.
        // "move" -> force ownership of value, but it doesn't work here as integer is a copy trait.
    };

    /*fn equal (z: i32) -> bool { // Needs to have definite x inside it.
        let x = 5;
        x == z
    }
    */
    println!("{}", x);
    let y = 4;
    assert_eq!(eql(y), true);

    let p = vec![1,2,3,4];
    let v = p.clone();

    let eql2 = move |z:Vec<i32>| {
        z == p
    };
    // println!("{:?}", p); Value moved

    let iter = v.iter(); // iter.next() -> moves 1 index to right
    for val in iter {
        // If you directly used `v` as `for val in v`, it would automatically call `v,into_iter()`.
        println!("{}", val);
    }
    // into_iter -> takes ownership and returns owned value (T)
    // iter_mut -> iterate over borrowed mutable collection,  returning you mut ref (&mut T)
}

pub trait Itera {
    type Item; // type returned from iter
    fn next(&mut self) -> Option<Self::Item>; // normally returns Some(&x) but none when over.
}

// Iterator Consumption -> Methods called consuming adaptors
fn iter_sum() {
    let v = vec![1,2,3];
    let vi = v.iter();
    let total = vi.sum(); // fn sum() exhausts takes ownership of`vi` and exhausts it.
}

// Change Iterators -> iteration adaptors

fn mapping() {
    let v = vec![1,2,3];
    let vi = v.iter().map(|x| x*2).collect::<Vec<i32>>();
    // collect -> exhausts iter => makes it functional as iter = lazy
}

// Creation of custom Iterators
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
