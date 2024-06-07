fn main(){
    // for i in 1..10{
    //     println!("{}",i);
    // }

    // println!("-----");
    // for i in 1..=10{
    //     println!("{}",i);
    // }
    // let nums=[1,3,5,7,9,11,13,15,17,19];
    // for i in nums{
    //     println!("{}",i);
    // }

    let n=5;
    let check_even_odd=if n%2==0{
        "Even"
    }else{
        "Odd"
    };
    println!("{}",check_even_odd);
}