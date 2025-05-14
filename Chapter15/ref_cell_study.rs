// Interior Mutability -> Mutate the data even when immutable references to data
// rules enforced at runtime, if rules' broken, panic and exit.
// Some issues -> Impossible to detect at compile time

fn main() {

}

pub trait Messenger {
    fn send(&self, msg: &str); // Send message without mutating themselves.
}
pub struct LimitTracker<'a, T: 'a + Messenger> { // T implements Messenger and lives long as 'a.
    messenger: &'a T,
    value: usize,
    max: usize,
}

// C# equivalent:

/*
interface Messenger
{
    void Send(string msg);
}

class LimitTracker
{
    private readonly Messenger messenger; // Reference to something implementing the interface

    public LimitTracker(Messenger msg)
    {
        messenger = msg;
    }
}

class LimitTracker<T> where T : Messenger
{
    private readonly T messenger;

    // This is equivalent to the 'new' function in the Rust implementation
    public static LimitTracker<T> New(T mes, uint max)
    {
        return new LimitTracker<T>
        {
            messenger = mes
        };
    }

*/

impl<'a, T> LimitTracker<'a, T>
where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

// There exists a trait Messenger with a function with 2 variables, &self and msg
// which allows the invocation of the function send without mutating themselves.
// We also have a public struct named LimitTracker which has lifetime annotation
// and a generic parameter which implements the trait Messenger and lives as long
// as 'a is alive. We have 2 variables under the struct with datatype usize because
// the size isn't known. The 3rd variable messenger lives as long as 'a and implements
// the trait i.e. gives it the functionality to send.  An implementation implements
// the struct with similar generic parameter and Lifetime Annotation. fn new is present
// which takes the reference to the trait Messenger datatype and a size  and returns the
// struct. Its main purpose is to create a new instance. The 2nd function is set_value
// which takes 2 parameter where one is usize datatype. Some conditions and
// arithmetic are done and the function defined at the trait Messenger is invoked.