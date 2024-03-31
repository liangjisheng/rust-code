use crossbeam_channel::{bounded, Receiver, Sender};

fn demo1() {
    let (tx, rx) = bounded(10);
    std::thread::spawn(move || {
        tx.send("hello").unwrap();
    });
    let msg = rx.recv().unwrap();
    println!("{}", msg);
}

fn try_send() {
    let (tx, rx) = bounded(10);
    std::thread::spawn(move || {
        loop {
            if let Err(_) = tx.try_send("hello") {
                // Channel is full, wait for a moment
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    });
    loop {
        match rx.recv_timeout(std::time::Duration::from_secs(1)) {
            Ok(msg) => {
                // Process the message
            }
            Err(_) => {
                // Channel is empty, wait for a moment
            }
        }
    }
}

fn main() {
    // demo1();
    try_send();
}
