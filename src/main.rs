#![allow(unused)]

//https://docs.rs/tokio/1.21.2/tokio/time/struct.Sleep.html
use log::{debug, info, warn};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let ms = 1_000;
    sleep(Duration::from_millis(ms)).await;
    log::error!("omg");
    log::info!("{}ms have elapsed - log", ms);
    println!("1000 ms have elapsed - println");

    let tasks: [u64; 8] = [1, 2, 3, 4, 5, 6, 7, 20];

    //sequential execution
    for task_seconds in tasks {
        long_running_task(task_seconds).await;
    }

    //parallel execution
    // for task_seconds in tasks {
    //     tokio::spawn(async move { long_running_task(task_seconds).await });
    // }

    // let (first, second) = tokio::join!(
    //     do_stuff_async(),
    //     more_async_work());

    //tokio::join ?
    //Task.WhenAll()
    //https://rust-lang.github.io/async-book/06_multiple_futures/02_join.html
    //https://docs.rs/tokio/latest/tokio/macro.join.html
    //https://stackoverflow.com/questions/63589668/how-to-tokiojoin-multiple-tasks

    println!("exiting - println");
    log::debug!("exiting - log");
}

async fn long_running_task(seconds: u64) {
    println!("long running fn delayed by {seconds}s - STARTED - println");
    log::debug!("long running fn delayed by {seconds}s - STARTED");
    sleep(Duration::from_secs(seconds)).await;
    //sleep(Duration::from_secs(seconds));
    //delay_for().await;
    println!("long running fn delayed by {seconds}s - COMPLETED - println");
    log::debug!("long running fn delayed by {seconds}s - COMPLETED");
}
