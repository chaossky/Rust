struct BagOfHolding<T>{
    // Option<T>
    item:Option<T>,
}
fn main() {
    let i32_bag = BagOfHolding::<i32>{item:None};
   
    if i32_bag.item.is_none(){
        println!("가방안에 아무 것도 없다.!");
    }else{
        println!("가방안에 무언가가 있다.!");
    }

    let i32_bag = BagOfHolding::<i32>{item:Some(42)};

    if i32_bag.item.is_some() {
        println!("가방안에 무언가가 있다.!");
    }else{
        println!("가방안에 아무 것도 없다.!");
    }

    match i32_bag.item{
        Some(v)=>println!("가방안에 {}를 찾았다.",v),
        None=>println!("가방안에 아무 것도 없다.!"),
    }
   
}
