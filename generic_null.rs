#[warn(dead_code)]
enum Item{
    Inventory(String),
    None,

}

struct BagOfHolding{
    item:Item,
}

fn main() {
    let nothing=BagOfHolding{item:Item::None};
    
}