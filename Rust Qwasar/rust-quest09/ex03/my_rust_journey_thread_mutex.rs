use std::sync::{Mutex, Arc};
use std::thread;

fn my_adder(mutex: &Mutex<i32>) {
    let mut data = mutex.lock().unwrap();
    *data += 1;
}

fn main() {
    let my_mutex = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let my_mutex = Arc::clone(&my_mutex);
        let handle = thread::spawn(move || {
            my_adder(&my_mutex);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("my_mutex => {:?}", my_mutex);
}
