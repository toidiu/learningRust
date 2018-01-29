use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

pub fn ch() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();


    for i in 0..NTHREADS {
        let thread_tx = tx.clone();

        thread::spawn(move || { thread_tx.send(i).unwrap(); });
    }

    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        ids.push(rx.recv());
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);

}
