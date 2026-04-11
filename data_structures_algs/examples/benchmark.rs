use data_structures_algs::dictionary::Dictionary;
use std::collections::HashMap;

fn main() {
    // benchmarking the custom hashmap versus std hashmap

    // standard library hashmap implementation
    let start = std::time::Instant::now();
    let mut std_hashmap: HashMap<i32, i32> = std::collections::HashMap::new();
    for i in 1..100_000 {
        std_hashmap.insert(i, i + 1);
    }
    let elapsed = start.elapsed();
    println!("Time elapsed std hashmap <INSERT>: {}", elapsed.as_micros());
    std_hashmap.get(&25);

    let start = std::time::Instant::now();
    let mut my_hashmap: Dictionary<i32, i32> = Dictionary::new();
    for i in 1..100_000 {
        my_hashmap.insert(i, i + 1);
    }
    let elapsed = start.elapsed();
    println!("Time elapsed my hashmap <INSERT>: {}", elapsed.as_micros());
    my_hashmap.get(&29);

    // getting with sum (to force compiler to not optimize things)
    let start = std::time::Instant::now();
    let mut sum: i64 = 0;
    for i in 1..100_000 {
        sum += *std_hashmap.get(&i).unwrap_or(&0) as i64;
    }
    let elapsed = start.elapsed();
    println!("Time elapsed std hashmap <GET>: {}", elapsed.as_micros());
    println!("{}", sum);

    let start = std::time::Instant::now();
    let mut sum: i64 = 0;
    for i in 1..100_000 {
        sum += *my_hashmap.get(&i).unwrap_or(&0) as i64;
    }
    let elapsed = start.elapsed();
    println!("Time elapsed my hashmap <GET>: {}", elapsed.as_micros());
    println!("{}", sum);
}
