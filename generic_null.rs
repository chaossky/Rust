#[warn(dead_code)]

#[derive(Debug)]
enum Item{
    Inventory(String),
    None,
}

#[derive(Debug)]
struct BagOfHolding{
    item:Item,
}

fn main() {
    let _nothing=BagOfHolding{item:Item::None};  
    let _sample=Item::Inventory("Potion".to_string()); 

    println!("{:?}",_nothing);
    println!("{:?}",_nothing.item);
    if let Item::Inventory(name) = _sample {
    println!("아이템 이름: {}", name);
    }
}