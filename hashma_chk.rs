use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);

    // if map.get("D")==None{
    //     println!("Not Found");
    // }else{
    //     println!("D={}",map["D"]);
    // }

    match map.get("A") {
        Some(v) => println!("A={}", v),
        None => println!("Not Found")
    }
    match map.get("D") {
        Some(v) => println!("D={}", v),
        None => println!("Not Found"),
    }


    }