mod liblists;
use liblists::*;

use tokio::time::{sleep, Duration};

// TODO: add functions to select inputs
async fn mainLogic() {
    sleep(Duration::from_secs(1)).await;
    println!("Async operation complete");
}

//
// TODO:
// - Get primary input list
// - Get secondary input list
// - Do things
//