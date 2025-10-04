fn main(){

    let grade='D';
    let message=match grade{
        'A'=>"Excellent",
        'B'=>"Good",
        'C'=>"Fair",
        'D'=>"Poor",
        _=>"Unknown",
    };
    println!("{}",message);
}
