use std::sync::mpsc::{Sender, Receiver};
use std::thread::JoinHandle;
use std::time::Duration;

// Sender function
fn sender(messages: Vec<String>, tx: Sender<String>) -> JoinHandle<()> {
    let handle = thread::spawn(move || {
        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    handle
}

// Receiver function
fn receiver(rx: Receiver<String>) -> JoinHandle<()> {
    let handle = thread::spawn(move || {
        for received in rx {
            println!("Received: {}", received);
        }
    });

    handle
}
