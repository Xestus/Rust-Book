mod main;

use std::sync::mpsc; // multiple producer single consumer
use std::thread;
use std::time::Duration;
// 2 halves: transmitter(release) and receiver(receive). Either dropped => "close"

fn main() {
    let (tx, rx) = mpsc::channel();

    // Multiple procedure:
    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let v = vec![1, 2, 3,4];
        for v in v {
            tx.send(v).unwrap(); // transfer thru channel
            thread::sleep(Duration::from_millis(1000));
        }
    }); // terminates

    thread::spawn(move || {
        let v = vec![9,8,7,6];
        for v in v {
            tx2.send(v).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    // let rec = rx.recv().unwrap();
    // blocks operation till value is available on channel. Unnecessary with loop.

    for rec in rx {
        println!("{:?}", rec);
    }
}

