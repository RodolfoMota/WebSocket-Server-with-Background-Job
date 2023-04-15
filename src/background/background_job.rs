use std::time::Duration;
use tokio::time;

pub async fn background_job() {
    let mut interval = time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        println!("Background job is running...");
    }
}
