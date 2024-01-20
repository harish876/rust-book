use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

fn main() {
    let num = Arc::new(Mutex::new(1));
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for i in 0..10 {
        let num = Arc::clone(&num);
        let handle: JoinHandle<()> = thread::spawn(move || {
            let mut val = num.lock().unwrap();
            *val += 1;
            println!("The value of number at thread {} is {}", i, val);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
