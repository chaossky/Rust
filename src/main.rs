fn main() {
    //rust는 snake_case를 사용핟다.
    // 동일한 이름의 변수에 여러 번 값을 할당 할 수 있다.
    // 변수 숨김(variable Shadowing)
    let x=13;
    println!("x={}",x);
    let x:f64=2.1234;
    println!("x={}",x);
    let x:i16;
    x=15;
    println!("x={}",x);
    let y:i16;
    y=112;
    println!("y={}",y);
}
