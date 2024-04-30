fn main() {
    let x=142;
    match x{
        0=>
        {
            println!("Zero");
        }
        1|2=>{
            println!("One or Two");
        }

        3..=9=>{
            println!("Three to Nine");
        }
        matched_num@ 10..=100=>{
            println!("10에서 100사이 숫자 {} 발견!",matched_num);
        }
        _=>{
            println!("Something else");        
        }
    }
}
