fn main() {
    let names = vec!["Rob", "뤼크", "😊Smile", "Alice"];

    for name in names {
        println!("---");
        println!("Name: {}", name);

        // 안전한 방식: chars().next()
        match name.chars().next() {
            Some(c) => println!("chars().next(): '{}'", c),
            None => println!("chars().next(): <None>"),
        }

        // 위험한 방식: name[0..1]
        // 이 방식은 UTF-8 문자 경계를 잘못 자르면 panic이 발생할 수 있음
        let slice_result = std::panic::catch_unwind(|| &name[0..1]);
        match slice_result {
            Ok(slice) => println!("name[0..1]: '{}'", slice),
            Err(_) => println!("name[0..1]: <panic 발생!>"),
        }
    }
}