fn main() {
    let s="3.1412121225";
    let num=s.parse::<f64>();
    match num{
        Ok(result)=>println!("{:2}",result),
        Err(e)=>println!("에러가 발생했습니다. 이유 :'{:?}'",e)
    }
   
}

//parse메서드의 반환값을 Result 타입이다.
/*
Result 타입에서 값을 출력하기 위해서는
expect메서드를 사용
변환에 실패하면 에러가 출력 되고 프로그램은 강제종료

Result 타입
<"데이터 타입","에러 타입"> 형태로 구성

Result<f64,ParseFloatError> 가 된다.

Result를 화면에 출력하는 방법은 다양
expect(에러 메시지)는
에러가 발생할 때 에러를 메시지를 출력하는 형태,
단순히 unwrap()메서드를 이용하는 방법도 있다.
이때 에러메시지를 표시되지 않는다.

println!매크로에서 "{}"대신 "{:?}" 를 
이용하는 방법도 있다.

이때는 Ok()안에 결과가 표시된다.

가장 좋은 방법은 match를 이용하는 것.

the `Result` type => an enum used for error handling.
(리절트 타입은 에러 처리를 위한 이넘타입이다.)
It represents a value that could be either successful or an error. 
(이것은 성공적이거나 에러일수 있는 값을 나타낸다.)


- `Result<T, E>`: This is the `Result` type, 
    where `T` is the type of the value returned in the case of a successful outcome,
      and `E` is the type of the error returned in the case of a failure.
- Variants:
  - **`Ok(T)`**: Indicates a successful result containing the value of type `T`.
  - **`Err(E)`**: Indicates an error, containing the error value of type `E`.

Here's a simple example to illustrate its use:



In this example, the `divide` function returns a `Result` type. If the denominator is not zero, it returns `Ok` with the division result. If the denominator is zero, it returns `Err` with an error message.

The `Result` type is particularly useful because it must be explicitly handled, either by pattern matching (as shown above) or by using methods like `unwrap()`, which either yields the successful value or panics if there's an error¹². Rust's compiler enforces handling of the `Result` to ensure that errors are not ignored, making your programs more robust.

원본: Copilot과의 대화, 2024. 6. 8.
(1) std::result - Rust. https://doc.rust-lang.org/std/result/.
(2) Result in std::result - Rust. https://doc.rust-lang.org/std/result/enum.Result.html.
(3) Understanding and Using the Result Type in Rust Programming | Rust Result Example. https://www.gyata.ai/rust/rust-result-example/.
(4) Mastering Error Handling with the Result Type in Rust. https://medium.com/@mbugraavci38/mastering-error-handling-with-the-result-type-in-rust-a5e6cc4032bf.
(5) Rust - Result Type - GeeksforGeeks. https://www.geeksforgeeks.org/rust-result-type/.


*/

