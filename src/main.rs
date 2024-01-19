mod libfunc;
mod liblists;
use libfunc::main_logic;

#[tokio::main]
async fn main() {
    main_logic().await;
    loop {
    }
}
