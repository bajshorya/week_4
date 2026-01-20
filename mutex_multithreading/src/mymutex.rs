use std::{
    ops::AddAssign,
    sync::{Arc, Mutex},
    thread::{self, scope, spawn},
};

pub fn mutexexample() {
    let score = Mutex::new(0u16);
    let unlocked_data = score.lock();
    let mut data = unlocked_data.unwrap();
    data.add_assign(21);
    println!("{:?}", data);
}
pub fn mutex_multithreaded_example() {
    let counter = Arc::new(Mutex::new(0u16));
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
pub fn mutex_multithreaded_example_2() {
    let score = Mutex::new(0u16);
    let myfun = || {
        let mut data = score.lock().unwrap();
        for i in 0..=5 {
            data.add_assign(i);
        }
    };
    let myfun2 = || {
        let mut data = score.lock().unwrap();
        for i in 0..=5 {
            data.add_assign(i);
        }
    };
    _ = scope(|s| {
        s.spawn(myfun);
        s.spawn(myfun2);
    });
    println!("{:?}", score.lock().unwrap());
}
