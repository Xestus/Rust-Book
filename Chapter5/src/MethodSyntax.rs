use std::cmp::Ordering;
#[derive(Debug)] // Print debugging info

struct Dimension {
    width: u64,
    height: u64,
}

impl Dimension {
    fn area(mut self) -> i64 {
        self.width *= self.height;
        self.width as i64
    }

    fn inverse(&self, moo: &Dimension) -> i64 {
    let mut x = woof(self.width,moo.width);
        let mut y = woof(self.height,moo.height);
        x + y
    }
}

fn main() {
    let mut rect = Dimension {width :30, height :30};
    println!("rect is {:?}", rect.area());
    // println!("use again: {:?}", rect.area());
    // ownership is moved to `area` method, which returns an i64 and is dropped. No one owns it anymore.
    // println!("use again: {:?}", rect.width); -> Area has the ownership of variable rect. Not accessible.

    let rect1 = Dimension {width: 20, height: 30};
    let rect2 = Dimension {width: 40, height: 40};
}

fn woof(a : u64, b : u64) -> i64 {
    match a.cmp(&b) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}
