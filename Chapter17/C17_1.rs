// Traits -> Dynamic dispatch -> Compiler can't determine your method at compile time
// At runtime, pointers inside trait object to know which method to call.

// Object safe trait -> return type isn't self & no generic parameters because:
    // self -> No info about concrete types. Same with generics.
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

pub struct Screen { // Generic only useful if homogenous types.
    pub cmp: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for cmp in self.cmp.iter() {
            cmp.draw();
        }
    }
}

impl Draw for Button { // This allows the user to be used where dyn Draw is expected.
    fn draw(&self) {

    }
}


fn main() {
    let screen = Screen {
        cmp: vec![
            Box::new(Button {
                width: 10,
                height: 10,
                label: String::from("OK"),
            })
        ]
    };

    // let s = Screen { cmp: vec![Box::new("hi")] }; -> No impl Draw for &str;

    screen.run();
}



// Inheritance: Child type to use parent type && reuse of code.
/*pub struct AV {
    num: Vec<i8>,
    average: f64,
}

impl AV {
    pub fn new(&mut self, value:i8) {
        self.num.push(value);
        self.update();

    }

    pub fn remove(&mut self) -> Option<i8> {
        match self.num.pop() {
            Some(val) => {
                self.update();
                Some(val)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update(&mut self) {
        let t = self.num.iter().sum::<f64>();
        self.average = t/self.num.len() as f64;
    }
}
*/