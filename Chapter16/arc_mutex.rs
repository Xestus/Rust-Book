/* Two main rule:
    - Acquire lock before using data.
    - Release the lock to let other threads access it.

        -> Control access to data regardless of availability of thread,
        -> lock() returns a guard that implements Deref and DerefMut to inner value.
    */

use std::sync::*;
use std::thread;
use std::rc::*;
// Single threaded.
fn single_thread() { // mutex -> Access of data to only one thread -> tracked by "lock"
    let a = Mutex::new(21);
    {
        let mut z = a.lock().unwrap();
        // lock() returns a MutexGuard<T> that is a smart pointer -> access to protected value
        *z = 11; //dereferenced to modify the protected value.
    } // z is dropped releasing the lock.
    println!("{:?}", a); // *a.lock().unwrap() to print value
}

fn main() {
    let c = Arc::new(Mutex::new(5));
    // 10 threads trying to grab ownership -> Err.
    // Rc isn't allowed to pass through threads because
    // Rc uses non-atomic operation to update its reference count
    // meaning increment/decrement aren't synced between threads.
    let mut h = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&c); // Arc is slower than Rc
        let ha = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        h.push(ha);
    }

    for ha in h {
        ha.join().unwrap();
    }
    println!("{:?}", *c.lock().unwrap());
}

// Why Rc Can't Be Shared Between Threads:
// Imagine you and your friend are playing with a toy that counts how many people are using it.
// When you pick it up, the number goes up by one. When you put it down, the number goes down
// by one. When the number reaches zero, the toy goes back in the toy box.
// Now imagine if you and your friend were in different rooms, and both tried to play with
// the same toy at the exact same time. You might both think you're the only one playing
// with it. One of you might put it away while the other is still playing! That would be
// very confusing and might break the toy.
