use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("msg1"),
            String::from("msg2"),
            String::from("msg3"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("t1"),
            String::from("t2"),
            String::from("t3"),
            String::from("t4"),
            String::from("t5"),
            String::from("t6"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for rcvd in rx {
        println!("Got : {}", rcvd);
    }
}
