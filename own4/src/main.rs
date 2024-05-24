/*
소유권의 이전
owner가 함수의 인자로 전달이 되면,
ownership은 그 함수의 매개변수로 이동이 된다.

move이후에는 원래의 함수에 있던 변수는 더 이상 
사용할 수 없다.

move중에는 owner값의 stack메모리 함수 호출의 
매개변수 stack 메모리로 복사 된다.
 */
struct Foo{
    x:i32
}
fn do_something(f:Foo){
    println!("{}",f.x);
    //f가 여기서 drop된다.
}
fn main() {
    let foo=Foo{x:42};
    //foo가 do_something으로 move된다.
    do_something(foo);
    //foo는 더 이상 사용할 수없다.
}
