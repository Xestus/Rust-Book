pub fn add(a: u64, b: u64) -> u64 {
    a + b
}


pub fn greet(name : &str) -> String {
    format!("Hello, {}!", name.to_string())
}

struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    fn area(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Guess {
    value: u32,
}

impl Guess {
    fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

// Private function can be tested.
fn internal1(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    mod integration_test;

    #[test]
    fn is_result_not_5() {
        assert_ne!(add(2,2), 5); // result != 5
    }

    #[test] // run single test with "cargo test <part of test fn>"
    fn is_result_4() {
        let result = add(2, 2);
        assert_eq!(result, 4); // result == 4
    }

    #[test]
    fn internal_test() {
        assert_eq!(internal1(5, 3), 8);
    }

    #[test]
    fn rectangle_area() {
        let a = Rectangle{width: 2, height: 2};
        let b = Rectangle{width: 3, height: 1};
        assert!(!a.area(&b)); // a.area(&b) != true
    }

    #[test]
    fn greeting() {
        let result = greet("Meow");
        assert_eq!(result, "Hello, Meow!");
        assert!(
            result.contains("Meow!"),
            "Greeting did not contain 'Meow!', value was `{}`", result // if failed use this.
        );
    }

    // test should_panic revise.
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 300.")]
    #[ignore] // ignores the test while "cargo test". Only ignored test can be run through "--ignored"
    fn guess() {
        Guess::new(300);
    }

}
