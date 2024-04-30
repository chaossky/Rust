//Rust 변수 변경 가능한지에 대해서 주의한다.
// mutable  변경가능 - mut 키워드로 표시한다.
// immutable 변경불가
fn main(){
    let mut x=42;
    println!("{}",x);
    x=13;
    println!("{}",x);
}
