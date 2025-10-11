struct BagOfHolding <T>{
    item:T,
}

fn main()
{
    let i32_bag=BagOfHolding::<i32>{item:42};
    let bool_bag=BagOfHolding::<bool>{item:true};
    let str_bag=BagOfHolding::<&str>{item:"Hello"};
    let float_bag=BagOfHolding{item:3.14};
    let bag_in_bag=BagOfHolding{
        item:BagOfHolding{item:"꽝!"}
    };

    println!("{:?}",i32_bag.item);
    println!("{:?}",bool_bag.item);  
    println!("{:?}",str_bag.item);
    println!("{:?}",float_bag.item);
    println!("{:?}",bag_in_bag.item.item);
}