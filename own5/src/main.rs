fn main() {
    let test=String::from("Testing1");
    test1(&test);
    test2(&test);
}
fn test1(name:&String){
    println!("First function is : {}",name);
}
fn test2(name2:&String){
    println!("Second function is : {}",name2)
}

