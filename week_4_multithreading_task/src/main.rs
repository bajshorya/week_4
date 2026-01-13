use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle1 = thread::spawn(move || {
        let val = String::from("value");
        tx.send(val).unwrap();
    });

    let handle2 = thread::spawn(move || {
        let val = rx.recv().unwrap();
        println!("revd:{:?}", val);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Mutex / Arc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
