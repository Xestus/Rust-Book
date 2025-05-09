use std::collections::HashMap;
use std::time::Duration;
use std::thread::sleep;
fn main() {
    let user_value = 15;
    let rng = 5;
    workout_generator(user_value, rng);

    let closure = |x| {x};
    let s = closure("Meow");
    // let t = closure(2); -> Doesn't work
}

enum MyEnum {
    Text(String),
    Byte(u32),
}

pub struct Close<T>
where T: Fn(u32) -> u32
{
    pub calculation: T, // Holds closure lazily
    pub value: Option<MyEnum>, // None by default
    pub A : HashMap<MyEnum, MyEnum>,
}

impl<T> Close<T>
where T: Fn(MyEnum) -> MyEnum
{
    pub fn new(calculation: T) -> Close<T> { // Stores the closure but no execution
        Close {
            calculation,
            value: None,
            A : HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: MyEnum) -> MyEnum {
        self.A.entry(arg).or_insert({
            let v = (self.calculation)(arg);
            self.value = Some(v);
            v
        });

        match self.value {
            Some(v) => v, // already computed, returns cache result
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v); // cache result
                v
            }
        }
    }
}

fn workout_generator (user_value:u32, rng: u32) {
    let mut e = Close::new(|num: u32| -> u32 { // {} unnecessary when 1 value & auto type declaration
        println!("Processing");
        sleep(Duration::from_secs(1));
        num
    }); // Calls closure more than necessary, not recommended. Hence, passed to the fn new of impl.
    if user_value < 25 {
        println!("Calm down and do {}", e.value(&user_value)); // e(user_value
    } else {
        if rng > 5 {
            println!("Calm up and down is {}", e.value(user_value));
        } else {
            println!("Calm down and sprint for {} minutes.", e.value(user_value));
        }
    }
}

// Bad as fn gets called, replaced by closure
fn sim (user_value:u32) -> u32 {
    println!("Processing");
    sleep(Duration::from_secs(1));
    user_value
}

