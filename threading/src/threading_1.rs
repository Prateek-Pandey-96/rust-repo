use std::{thread,time::Duration};

pub fn threading_1(){
    let handle = thread::spawn(||{
        for i in 1..5{
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3{
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}