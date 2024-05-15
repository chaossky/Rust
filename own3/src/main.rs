struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f가 여기서 drop 됩니다
}

fn main() {
    let foo = Foo { x: 42 };
    // foo가 do_something으로 move 됩니다
    do_something(foo);
    // foo는 더 이상 사용할 수 없습니다
}