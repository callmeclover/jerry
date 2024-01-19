use crate::liblists::*;
use enigo::Key;
use tokio::time::{sleep, Duration};

fn get_weighted_index(list: Vec<(&'static str, usize)>) -> WeightedIndex<usize> {
    return WeightedIndex::new(list.iter().map(|item| item.1)).unwrap();
}

fn get_weighted_index_keys(list: Vec<(Key, usize)>) -> WeightedIndex<usize> {
    return WeightedIndex::new(list.iter().map(|item| item.1)).unwrap();
}

// TODO: add functions to select inputs
pub async fn main_logic() {
    sleep(Duration::from_secs(1)).await;
    
    //let options = get_config_options();
    let dist = get_weighted_index(options);

    sleep(Duration::from_secs(1)).await;
}