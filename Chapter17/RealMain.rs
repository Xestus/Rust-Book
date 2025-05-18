use std::cmp::PartialEq;

struct Post {
    state: State ,
    hold : String,
    content: String,
}

enum State {
    New,
    Approved,
    Requested,
    Added,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (State::Added, State::Added) => true,
            (State::New, State::New) => true,
            (State::Requested, State::Requested) => true,
            (State::Approved, State::Approved) => true,
            _ => false,
        }

    }
}

impl Post {
    fn new() -> Post {
        Post {
            state: State::New,
            hold : String::new(),
            content: String::new(),
        }
    }


    fn add_text(&mut self, text: &str) {
        self.hold.push_str(text);
        self.state = State::Added;
    }

    fn check(&mut self) {
        if self.state == State::Approved {
            self.content = self.hold.clone();
        }
    }
    fn content(&self) -> &str {
        &self.content
    }

    fn request_review(&mut self) {
        self.state = State::Requested;
    }

    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = State::Approved;
        }
        self.check();
    }
}
fn main() {
    let mut p = Post::new();

    p.add_text("Hello");
    assert_eq!(p.content(), "");

    p.request_review();
    assert_eq!("", p.content());

    p.approve();
    assert_eq!(p.content(), "Hello");
}