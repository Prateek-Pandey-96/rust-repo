use std::{sync::mpsc, thread, time::Duration};

pub fn threading_3(){
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    
    thread::spawn(move||{
        let messages = vec![
            "my",
            "name",
            "is",
            "Prateek"
        ];

        for message in messages{
            tx.send(message).unwrap();
        }
    });

    thread::spawn(move||{
        let messages = vec![
            "I",
            "live",
            "in",
            "Bengaluru"
        ];

        for message in messages{
            tx2.send(message).unwrap();
        }
    });

    for message in rx{
        println!("{}", message);
        thread::sleep(Duration::from_secs(1));
    }
}