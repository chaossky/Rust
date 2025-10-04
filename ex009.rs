fn main(){
    let words="human being alive go";

    for token in words.split_whitespace(){
        println!("{}",token);
    }

    let sentence_to_words="Don’t kill the goose that lays the golden eggs.";
    println!("{}",&sentence_to_words);
    for token in sentence_to_words.split_whitespace(){
        println!("{}",token);
    }

    let word=String::from("Hello World");
    // 소유권을 넘기지 않고 출력
     println!("{}",&word);//참조를 넘김
     // &는 참조를 명시적으로 넘겨주는 방식
     // Rust의 안정성과 효율성을 유지하는데 중요
    // 소유권을 넘기고 출력
    println!("{}",word);
   /*

        &str :	문자열 슬라이스. 읽기 전용, 고정된 크기.
        String :	힙에 저장된 가변 문자열. 수정 가능.

        "hello"는 기본적으로 &str 타입입니다.
        String은 &str보다 더 유연하며, 문자열을 추가하거나 수정할 수 있습니다.

        let word = String::from("hello");
        "hello"라는 &str을 String으로 변환합니다.
        이로 인해 word는 String 타입이 되어 .push_str(), .len(), .to_uppercase() 같은 메서드를 사용할 수 있게 됩니다.

        to_string() 메서드도 같은 역할을 합니다:

        rust
        let s = "hello".to_string(); // &str → String
        둘 다 자주 쓰이며 기능적으로 거의 동일하지만, String::from()은 더 명시적인 타입 변환 느낌을 줍니다
    */


}
