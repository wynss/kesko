use renet_transport::RenetServerWorker;
use std::{thread, time::Duration};

fn main() {
    let mut worker_thread = RenetServerWorker::new();

    for i in 0..100000000 {
        worker_thread.send(format!("Message {}", i));
        thread::sleep(Duration::from_millis(1000)); // Add a small delay here
        if let Some(response) = worker_thread.try_receive() {
            println!("Response from worker: {}", response);
        }
    }

    worker_thread.join();
}
