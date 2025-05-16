use std::thread;
use std::time::*;

fn main() {
    let h = thread::spawn(|| {
        for j in 1..11 {
            println!("Spawn: {}",j);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for j in 1..4 {
        println!("Main: {}",j);
        thread::sleep(Duration::from_millis(1));
    } // Spawn thread stops when main thread is over,
    h.join().unwrap(); // j complete run. Blocks every thread except h.

    let v = vec![1,2,3];

    let a = thread::spawn(move || {
        // tries to borrow v to print, there is no exact duration of thread, CRASH.
        println!("Vector: {:?}", v);
    });
    drop(v);  // chance of main thread, run main thread, dop main thread. Thread a has nothing to run.
    // Adding move on thread passes the ownership to a, v not being dropped.

    a.join().unwrap();

}