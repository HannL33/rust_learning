use std::collections::HashMap;

fn main() {
    let mut my_hash = HashMap::new();
    my_hash.insert("asd", 1.0);
    println!("{:?}", my_hash);
    my_hash.entry("asd").or_insert(2.0);

    println!("{:?}", my_hash.contains_key("asd"));
    // my_hash.insert(123, 123);  -> cannot do that, as hashmaps are generic but fixed types
    // println!("{:?}", my_hash);
}
