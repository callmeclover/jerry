mod libfunc;
use libfunc::*;
use keyboard::*;

#[tokio::main]
async fn main() {
    while true {
        mainLogic().await;
    };
}