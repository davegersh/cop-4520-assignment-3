use std::sync::{RwLock, Arc};
use std::thread;
use std::time::Duration;

fn main() {
    let shared_data = Arc::new(RwLock::new(Vec::<i32>::new()));

    let shared_data_reader = shared_data.clone();
    let reader_thread = thread::spawn(move || {
        let shared_data_reader = shared_data_reader.read().unwrap();
        println!("Reader Thread: {:?}", *shared_data_reader);
    });

    let shared_data_writer = shared_data.clone();
    let writer_thread = thread::spawn(move || {
        {
            let mut shared_data_writer = shared_data_writer.write().unwrap();
            shared_data_writer.push(42);
        }

        thread::sleep(Duration::from_secs(1));
        println!("writer done!");
    });

    reader_thread.join().expect("Reader thread panicked");
    writer_thread.join().expect("Writer thread panicked");

    let final_data = shared_data.read().unwrap();
    println!("Final Data: {:?}", *final_data);
}