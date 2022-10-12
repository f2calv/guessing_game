struct TestStruct{

}

impl TestStruct {
    pub fn new(
    ) -> Self {
        TestStruct { }
    }

    async fn long_running_task(seconds: u64) {
        let duration = time::Duration::from_secs(seconds);
        println!("long running fn delayed by {seconds}s - STARTED");
        thread::sleep(duration);
        println!("long running fn delayed by {seconds}s - COMPLETED");
    }
}