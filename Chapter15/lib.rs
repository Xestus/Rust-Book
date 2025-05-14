mod ref_cell_study;
pub use ref_cell_study::*;
use std::cell::RefCell;
#[cfg(test)]
mod tests {
    use super::*;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
           self.sent_messages.borrow_mut().push(String::from(message)); //borrow_mut() creates mutable reference
            // self.sent_messages.push(String::from(message)); Doesn't work as `send` takes immut value.
            // Cannot be modified as Messenger trait takes immut value.

            let mut one = self.sent_messages.borrow_mut();
            one.push(String::from(message));
            // Error because 2 mut reference in 1 scope
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); // .borrow to create immutable reference
    }
}
