use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's the vector: {:?}", v);
    });

    handle.join().unwrap();
}
