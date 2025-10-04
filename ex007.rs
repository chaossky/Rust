fn example()->i32{
    let x=42;
    let v=if x>30 {-1}else{1};
    println!("if 로 부터 {}",v);
    
    let food="apple";
    let result=match food{
        "apple"=>"this is apple.",
        "grape"=>"This is grape.",
        _=>"Unknown",
    };
    println!("음식 판별 결과:{}",result);
    
    let v={
        let a=1;
        let b=2;
        a+b
    };
    println!("블록에서 v={}",v);

    v+4
}

fn main(){
    println!("function() 에서 : {}", example())
}
