pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod client;
pub mod network;

mod outermost {
    pub mod inside {
        pub mod deep {
            pub fn in_fn() {}
        }
        fn sec_fn() {}
    }

    // No code can be placed outside the fn:         in_fn(); -> Error
    fn invoke_mid_fun() {
        inside::deep::in_fn();
    }
}

// PRivacy rules apply to items inside modules, not to modules themselves inside the same crate.
fn meow() {
    outermost::inside::deep::in_fn(); //Long

    // Short
    use outermost::inside::deep::in_fn;
    in_fn();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
