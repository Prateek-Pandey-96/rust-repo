use std::sync::Arc;
use std::{thread, sync::Mutex};

pub fn threading_4(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..5{
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handler);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}