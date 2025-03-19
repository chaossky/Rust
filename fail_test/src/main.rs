fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("맞는 숫자가 아닙니다"))
    }
}
// Main은 아무 값도 리턴하지 않지만, 오류를 리턴할 수 있다!
fn main() -> Result<(), String> {
    let result = do_something_that_might_fail(12);
    match result {
        Ok(v) => println!("{} 발견", v),
        Err(_e) => {
            // 이 오류를 우아하게 처리한다
            // main으로부터 무슨 일이 발생했는지 새 오류를 리턴한다!
            return Err(String::from("main에서 뭔가 잘못 되었습니다!"));
        }
    }
    // 모든 일이 잘 끝났음을 표현하기 위해
    // Result Ok 안에 unit 값을 쓰고 있는걸 잘 봐두십시오
    Ok(())
}