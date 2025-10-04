/*
Tuple 같은 구조체
간결함을 위해, tuple처럼 사용되는 struct를 생성할 수 있습니다.
*/

struct Location(i32, i32);

fn main() {
    let loc = Location(10, 20);
    println!("x: {}, y: {}", loc.0, loc.1);
}