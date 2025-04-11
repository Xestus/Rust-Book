fn main() {
    let mut s = String::from("hello");
    {
        // Only one can be mutated at a time. Prevents data race i.e.
        // 2+ ptrs access same data @ same time, 1 of ptr being used to write data.
        // No synchronization of data access.
        { // r1 -> Out of scope.
            let r = &s; // If this is added, r1 doesn't work as you can have either:
            // 1 mutable reference or inf immutable reference.
            // because it's reading the data, not editing the data.
            let r1 = &mut s;
        }
        let r2 = &mut s;
    }
    {
        let s3 = ae(&s);
        me(&mut s); // Only 1 at a time
        println!("HI {} from s3", s3);
        println!("HI {} from s", s);
        let x: (i32, f64, u8) = (500, 6.4, 1);
        let five_hundred = x.0;
    }



}
fn ae(meow: &String) -> i32 {
    println!("{}", meow);
    // meow.push_str(", world");
    // even though original string is mutable it doesn't work as
    // we're borrowing the data as immutable references
    5
}

fn me(meow: &mut String) {
    meow.push_str(", world");
}

fn dangle() -> String /* &String */ {
    let s = String::from("hello");
    s   /* &s */
    // commented doesn't work as when the reference is returned,
    // 's' will go out of scope and will be dropped. Hence, no value to point to.
}
