// Import the necessary modules
use tokio::time::Duration;
use tokio::task;

// A simple asynchronous function that waits for a given number of seconds
async fn wait_for_seconds(seconds: u64) {
    // Create a delay using Tokio's sleep function
    // The sleep function returns a future that completes after the specified duration
    println!("Waiting for {} seconds...", seconds);
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    
}

#[tokio::main]
async fn main() {
    // Call the asynchronous function using `.await`
    let res = wait_for_seconds(3).await;
    println!("{:?}", res);
    println!("Async function call completed!");
}