// Mutex that protects the data vector, and then we spawn three threads 
//that each acquire a lock on the mutex and modify an element of the vector.

use std::thread;
//use std::sync::{Arc, Mutex};
use std::sync::{Arc, RwLock};

fn main() {
    //let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    //we can use RwLock instead of Mutex
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    let handles: Vec<_> = (0..3).map(|i| {
        let data = data.clone();
        thread::spawn(move || {
            //let mut data = data.lock().unwrap();

            //instead of .lock() for Mutex - use .write() for RwLock
            let mut data = data.write().unwrap();
            data[i] += 1;
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data);
}


/*
use std::thread;

fn main() {
    let mut data = vec![1, 2, 3];

    for i in 0..3 {
        // Try to capture a mutable reference in multiple threads
        // This will fail to compile!
        thread::spawn(move || {
            data[i] += 1;
        });
    }

    // No data race can occur, this will not compile.
}
 */