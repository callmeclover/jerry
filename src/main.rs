mod libfunc;
mod libconf;
//mod liblists;
use libconf::*;
use libfunc::main_logic;

//use tts::*;
use enigo::*;

#[tokio::main]
async fn main() {
    let config = get_config().await;
    let options = get_options(config).await;

//    let mut tts = Tts::default();
    let mut enigo = Enigo::new();

    loop {
        main_logic(&options, /*tts, */&mut enigo).await;
    }
}
