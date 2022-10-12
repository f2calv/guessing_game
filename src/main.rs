#![allow(unused)]

use std::{
    thread,
    time::{self, Duration},
};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let tasks: [u64; 8] = [1, 2, 3, 4, 5, 6, 7, 20];

    //sequential execution
    for seconds in tasks {
        long_running_task(seconds).await;
    }
}

async fn long_running_task(seconds: u64) {
    let duration = time::Duration::from_secs(seconds);

    println!("long running fn delayed by {seconds}s - STARTED");
    thread::sleep(duration);
    println!("long running fn delayed by {seconds}s - COMPLETED");
}
