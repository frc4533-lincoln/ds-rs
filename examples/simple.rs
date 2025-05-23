extern crate ds;

use ds::*;

use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let ds = DriverStation::new_team(4533, Alliance::new_red(1)).await;

    thread::sleep(Duration::from_millis(1500));
    loop {
        println!("Code: {}", ds.trace().await.is_code_started());

        thread::sleep(Duration::from_millis(20));
    }
}
