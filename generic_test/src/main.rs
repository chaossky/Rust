struct BagOfHolding<T>{
    // T:Copy,제너릭 
    item:T,
}
fn main() {
    let i32_bag=BagOfHolding::<i32>{item:42};
    let bool_bag=BagOfHolding::<bool>{item:true};
    let float_bag=BagOfHolding{item:3.14};

 //   let bag_in_bag=BagOfHolding{
 //       item:BagOfHolding{item:"꽝"}
 //   };

    print!(
        "{} {} {} ",
        i32_bag.item,bool_bag.item,float_bag.item
              );
  
}
