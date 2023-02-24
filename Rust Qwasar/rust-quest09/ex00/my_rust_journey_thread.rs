use std::thread::JoinHandle;

fn born_to_be_alive() -> JoinHandle<()> {
    let handle = std::thread::spawn(|| {
        for _i in 0..10 {
            println!("Born to be alive");
        }
    });

    handle
}
