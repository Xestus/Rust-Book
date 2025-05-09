mod main;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut c = main::Close::new(|v| v);
        let mut a = c.value(1);
        let mut b = c.value(2);

        assert_eq!(2, b);
    }
}