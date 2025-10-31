use std::time::Duration;

use tokio::select;

async fn sync_tick() {
    println!("1");
    tokio::time::sleep(Duration::from_secs(1)).await;
}

async fn sync_tick2() {
    println!("2");
    tokio::time::sleep(Duration::from_secs(2)).await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        select! {
            _ = sync_tick() => {},
            _ = sync_tick2() => {},
        }
    }
}
