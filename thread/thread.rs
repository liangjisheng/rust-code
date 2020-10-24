use std::thread;
use std::time::Duration;

fn main() {
    // create a new thread
    thread::spawn( || {
        for i in 1..10 {
            println!("thread: number {}!", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    thread::sleep(Duration::from_millis(1000));
    println!("hi from the main thread!")
}
