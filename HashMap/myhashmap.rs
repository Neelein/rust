use std::collections::{HashMap,HashSet};

pub fn test_hashmap_basic(){
    let stock_list:HashMap<String,f32> = HashMap::<String,f32>::new();
    println!("{}",stock_list.len());
}