//일부만 정의된 struct 자료형
struct BagOfHolding<T>{
    //자료형의 T의 인자는 다른 곳으로 넘겨 질 수 있음
    item:Option<T>,
}

fn main() {
    // 중요: i32를 위한 가방에 아무 것도 안들었음
    // Rust가 무슨 자료형의 가방인지를 알수 없다.
    // 따라서 자료형을 지정해야 함.
    let i32_bag=BagOfHolding::<i32>{item:None};

    if i32_bag.item.is_none(){
        println!("가방안에 아무것도 없다.");
    }else{
        println!("가방안에 무언가가 있다.");
    }

    let i32_bag=BagOfHolding::<i32>{item:Some(42)};

    if i32_bag.item.is_some(){
        println!("가방안에 무언가가 있다.");
    } else{
        println!("가방안에 아무것도 없다.");
    }

    //match는 Option을 우아하게 분해하고, 모든 케이스를 처리하도록 해준다.
    match i32_bag.item{

        Some(v)=>println!("가방안에 {}를 찾았다.",v),
        None=>println!("가방안에 아무것도 없다."),
    }
}