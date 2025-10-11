use std::fmt;

struct Person {
    name: String,
    age: u8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old.", self.name, self.age)
    }
}

fn main() {
    let p = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("{}", p); // Alice (30)
}

/*
starts_with()는 Rust에서 문자열이 특정 접두사로 시작하는지를 검사하는 문자열 메서드

🔍 .starts_with()란?
문자열이 특정 문자열 또는 문자로 시작하는지 여부를 확인

let name = "Rust";
println!("{}", name.starts_with("Ru")); // true
println!("{}", name.starts_with("rust")); // false (대소문자 구분)
반환값은 bool (true 또는 false)

대소문자를 구분합니다

검사 대상은 &str 또는 char 타입

📦 사용 예시
1. 문자열 접두사 검사

let s = "hello world";
assert!(s.starts_with("hello")); // true
2. 단일 문자 검사

let name = "Rachel";
if name.starts_with('R') {
    println!("{} plays banjo", name);
}
3. 조건 분기

fn greet(name: &str) {
    if name.starts_with("Dr.") {
        println!("Hello, doctor!");
    } else {
        println!("Hello, {}!", name);
    }
}
✅ 특징 요약
항목	설명
메서드 이름	.starts_with()
소속 타입	&str
인자 타입	&str 또는 char
반환값	bool
대소문자	구분함

🧠 실전 활용: banjo 예제

fn are_you_playing_banjo(name: &str) -> String {
    if name.starts_with('R') || name.starts_with('r') {
        format!("{} plays banjo", name)
    } else {
        format!("{} does not play banjo", name)
    }
}
이름이 'R' 또는 'r'로 시작하면 banjo를 연주한다고 판단합니다.

궁금하시면 .ends_with()나 .contains() 같은 관련 메서드도 이어서 설명해드릴게요!

이것은 아스키나 유니코드나 상관이 없나요?
좋은 질문이에요! .starts_with()는 아스키(ASCII)든 유니코드(UTF-8)든 상관없이 잘 작동합니다. Rust의 문자열은 UTF-8로 인코딩되어 있기 때문에, .starts_with()는 내부적으로 문자열의 바이트가 아닌 문자 단위로 비교를 수행합니다.

✅ 핵심 요점
항목	설명
지원 문자	ASCII 문자, 한글, 이모지 등 모든 유니코드 문자
비교 방식	문자열의 앞부분이 주어진 문자열 또는 문자와 일치하는지 확인
대소문자	구분함 ("Rust".starts_with("r") → false)
안전성	UTF-8 경계를 자동으로 처리하므로 panic 발생 없음
🧪 예시: 유니코드 문자열에서도 안전하게 작동

fn main() {
    let korean = "안녕하세요";
    let emoji = "😊Smile";

    println!("{}", korean.starts_with("안")); // true
    println!("{}", emoji.starts_with("😊")); // true
    println!("{}", emoji.starts_with("S"));  // false
}
"안녕하세요"는 한글이지만 "안"으로 시작하는지 정확히 판단합니다.

"😊Smile"도 이모지로 시작하는지 문제없이 검사합니다.

🧠 내부 동작 간단히
.starts_with()는 문자열의 앞부분을 UTF-8 문자 단위로 슬라이스해서 비교합니다.

Rust는 UTF-8 경계를 자동으로 처리하므로, 직접 바이트 인덱스를 다룰 필요가 없습니다.

결론적으로, .starts_with()는 아스키든 유니코드든 모두 안전하고 정확하게 작동합니다. 혹시 .ends_with()나 .contains() 같은 관련 메서드도 궁금하신가요?

예를 들어 "안녕"에서 'ㅇ' 로 시작하는지는 가능한가요?
좋은 질문이에요! "안녕"이라는 문자열이 'ㅇ'으로 시작하는지를 .starts_with()로 검사하는 건 불가능합니다, 왜냐하면 "안녕"의 첫 글자인 "안"은 한 글자 전체가 하나의 유니코드 문자이기 때문이에요.

🔍 핵심 개념
"안녕"의 첫 글자 "안"은 하나의 완전한 문자입니다.

'ㅇ'은 그 글자를 구성하는 자모(초성)이지만, Rust에서는 "안"을 'ㅇ'과 'ㅏ'와 'ㄴ'으로 나누지 않습니다.

따라서 "안녕".starts_with('ㅇ')는 false를 반환합니다.

✅ 확인 예제

fn main() {
    let s = "안녕";

    println!("{}", s.starts_with('안')); // true
    println!("{}", s.starts_with('ㅇ')); // false
}
"안"은 유니코드 문자 하나이므로 starts_with('안')은 true

'ㅇ'은 "안"의 초성일 뿐이므로 starts_with('ㅇ')은 false

🧠 만약 초성으로 비교하고 싶다면?
Rust 표준 라이브러리에는 초성/중성/종성을 분리하는 기능이 없지만, 한글 자모 분리 라이브러리를 사용하면 가능합니다. 예를 들어 hangul 같은 크레이트를 사용하면 "안"을 'ㅇ', 'ㅏ', 'ㄴ'으로 분해할 수 있어요.

필요하시면 한글 자모 분리 예제도 Rust 코드로 보여드릴게요! 초성 검색이나 한글 필터링 기능을 만들고 싶으신가요?

hangul 이라는 크레이트에 대해서 정보를 얻을 수 있는 사이트를 알려주세요.
hangul 크레이트에 대한 자세한 정보는 Rust 공식 문서 사이트인 Docs.rs에서 확인할 수 있어요. 아래 링크를 통해 직접 접근하실 수 있습니다:

🔗 hangul - Rust documentation on Docs.rs

이 크레이트는 한글 음절을 다루기 위한 다양한 기능을 제공합니다. 예를 들어:
한글인지 여부 확인
초성, 중성, 종성 분리
자모 단위 반복자 제공
한글 문자열을 세밀하게 분석하거나 초성 검색 기능을 구현할 때 매우 유용

--------------------------------------------------------------
.nth()는 Rust에서 반복자(iterator)를 다룰 때 사용하는 메서드
n번째 요소를 가져오는 기능을 합니다. 

🔍 .nth(n)란?
.nth(n)는 반복자에서 n번째 요소를 꺼내서 반환

n은 0부터 시작하는 인덱스

반환값은 Option<T> 타입:

Some(value) → 해당 위치에 값이 있을 때

None → 범위를 벗어났을 때

📦 예시: 문자열에서 문자 꺼내기

let name = "Rust";
let second_char = name.chars().nth(1);
println!("{:?}", second_char); // Some('u')
name.chars()는 문자열을 문자 단위로 반복하는 반복자

.nth(1)은 두 번째 문자를 꺼냄 ('u')

반환값은 Option<char>

🧠 중요한 특징
항목	설명
반복자 소모	.nth(n)를 호출하면 앞의 n개 요소는 버려지고, n번째 요소만 반환됨
한 번만 사용 가능	
반복자는 소모되므로, 같은 반복자에서 여러 번 .nth()를 호출하면 결과가 달라짐
안전성	UTF-8 문자 단위로 처리되므로 한글, 이모지 등도 정확하게 작동함

🎯 실전 예시: banjo 함수

fn are_you_playing_banjo(name: &str) -> String {
    match name.chars().nth(0) {
        Some('r') | Some('R') => format!("{} plays banjo", name),
        _ => format!("{} does not play banjo", name)
    }
}
name.chars().nth(0)은 문자열의 첫 번째 문자를 가져옵니다

그 문자가 'r' 또는 'R'이면 banjo를 연주한다고 판단

그렇지 않으면 연주하지 않는다고 출력

✅ 요약
.nth(n)	반복자에서 n번째 요소를 꺼냄
반환값	Option<T> (Some(value) 또는 None)
사용 예	문자열에서 특정 위치의 문자 가져오기

--------------------------------------------------------------------------------
Some('R' | 'r') 이 구문과 Some('R') | Some('r') 이 구문은 같은 건가요?
아주 중요한 차이가 있는 질문이에요! 결론부터 말하면:

❌ Some('R' | 'r') 와 Some('R') | Some('r')는 같은 의미가 아닙니다.

Some('R' | 'r')
이 표현은 비트 연산자 |를 사용한 것입니다.

'R'와 'r'는 char 타입이지만, 내부적으로는 유니코드 코드 포인트(숫자)로 표현됩니다.

따라서 'R' | 'r'는 두 숫자를 비트 OR 연산한 결과가 됩니다.

그 결과는 새로운 char가 되며, 'R'이나 'r'과는 전혀 다른 문자일 수 있어요.

예시:

println!("{}", 'R' as u32); // 82
println!("{}", 'r' as u32); // 114
println!("{}", ('R' | 'r') as u32); // 114 (결과는 'r')
Some('R' | 'r')는 결국 Some('r')와 같아지는 경우도 있지만, 의도한 비교 방식이 아닙니다.

2. Some('R') | Some('r')
이 표현은 논리 비교에서 사용하는 패턴 매칭입니다.

match 구문에서 Some('R') | Some('r')는 두 가지 경우 중 하나에 해당하면 매칭됩니다.

즉, 'R' 또는 'r'일 때 모두 해당됩니다.

예시:

match Some('R') {
    Some('R') | Some('r') => println!("Matched!"),
    _ => println!("No match."),
}
결과: "Matched!"

✅ 올바른 사용법

match name.chars().nth(0) {
    Some('R') | Some('r') => format!("{} plays banjo", name),
    _ => format!("{} does not play banjo", name),
}
이 방식이 정확하고 안전한 비교입니다.

Some('R' | 'r')는 절대 사용하지 마세요 — 의도와 다른 결과를 낼 수 있어요.

🧠 요약
표현	의미	추천 여부
`Some('R'	'r')`	비트 OR 연산 → 예기치 않은 문자	❌ 사용 금지
`Some('R')	Some('r')`	패턴 매칭 → 둘 중 하나	✅ 안전하고 정확

Some('R' | 'r') 이것은 'r' 또는 'R' 이 있는지를 판단하기 보다는 
두 char 타입을 비트 연산한 결과로 인식할 수 있다는 말인가요?
정확히 이해하셨어요! 👏

Some('R' | 'r')는 Rust에서 논리적 "또는" (OR) 비교가 아니라, 
비트 연산자 |를 사용한 연산으로 해석됩니다. 이건 매우 중요한 차이예요.

🔍 Some('R' | 'r')의 실제 의미
'R'과 'r'는 각각 char 타입입니다.

Rust에서 char는 내부적으로 유니코드 코드 포인트, 즉 u32 정수로 표현됩니다.

'R' | 'r'는 두 u32 값을 비트 OR 연산한 결과를 다시 char로 변환한 것입니다.

예시로 확인해볼까요?

fn main() {
    let r_upper = 'R' as u32; // 82
    let r_lower = 'r' as u32; // 114
    let result = (r_upper | r_lower) as char;

    println!("'R' as u32: {}", r_upper); // 82
    println!("'r' as u32: {}", r_lower); // 114
    println!("'R' | 'r' = {}", result);  // 출력: 'r'
}
82 | 114 = 114 → 결과는 'r'

즉, Some('R' | 'r')는 항상 Some('r')로 평가됩니다.

따라서 'R'일 경우에는 매칭되지 않습니다!

✅ 올바른 방식: 패턴 매칭에서 |는 패턴 분기

match Some('R') {
    Some('r') | Some('R') => println!("Matched!"),
    _ => println!("No match."),
}
이건 'r'이거나 'R'일 때 모두 매칭됩니다.

|는 비트 연산이 아니라 패턴 분기로 해석됩니다.

🧠 요약
표현	의미	결과
`Some('R'	'r')`	비트 OR 연산 → Some('r')	❌ 의도와 다름
`Some('R')	Some('r')`	패턴 분기 → 둘 중 하나 매칭	✅ 올바른 방식
Rust는 타입이 명확하고 연산자 의미가 엄격해서 이런 실수를 방지하기 위해 컴파일러가 경고를 주기도 해요. 
하지만 의미를 정확히 아는 것이 가장 중요하죠. 
-------------------------------------------------------------------
.to_owned()는 Rust에서 값을 복사해서 소유권을 가져오는 메서드입니다. 
특히 &str 같은 참조 타입을 String으로 변환할 때 자주 사용.

🔍 .to_owned()란?
.to_owned()는 Clone 트레이트를 구현한 타입에서 사용할 수 있는 메서드

참조(&T)를 소유하는 값(T)으로 복사.

문자열에서는 &str → String으로 변환하는 데 자주 쓰입니다.

📦 예시

let s = "Rust";           // &str
let owned = s.to_owned(); // String
s는 문자열 리터럴 (&str)

owned는 힙에 저장된 String 타입

결과적으로 owned는 s의 복사본을 소유하게 됩니다

✅ 왜 필요한가요?
Rust는 소유권 시스템을 갖고 있어서, 참조(&str)는 임시로 빌려온 값이에요. 
하지만 String은 자기 자신이 메모리를 소유하므로, 다음과 같은 작업이 가능해집니다:

값을 수정하거나 연결하기 (+, push_str)
다른 스코프에서 안전하게 사용하기
반환하거나 구조체에 저장하기

🎯 실전 예시 분석

fn are_you_playing_banjo(name: &str) -> String {
    if name.starts_with(&['R', 'r']) {
        name.to_owned() + " plays banjo"
    } else {
        name.to_owned() + " does not play banjo"
    }
}
name은 &str 타입 → 참조
name.to_owned() → String으로 변환

+ " plays banjo" → 문자열 연결은 String 타입에서만 가능

📌 만약 .to_owned() 없이 name + "..."를 하려고 하면 컴파일 오류가 납니다. 
왜냐하면 &str에는 + 연산자가 정의되어 있지 않기 때문이에요.

🧠 .to_owned() vs .to_string()
메서드	설명	차이점
.to_owned()	참조를 복사해서 소유권을 가짐	더 일반적 (Clone 기반)
.to_string()	문자열을 String으로 변환	Display 트레이트 기반
둘 다 &str → String 변환에는 동일하게 작동하지만,
 .to_owned()는 더 넓은 타입에 적용 가능해요.
-------------------------------------------------------
🔍 .unwrap()이란?
.unwrap()은 Option<T> 또는 Result<T, E> 타입에서:

값이 있으면 → Some(value) 또는 Ok(value)에서 그 값을 꺼냄
값이 없으면 → None 또는 Err(e)일 경우 프로그램이 panic(강제 종료)

📦 예시: Option에서 사용

let name = "Rust";
let first_char = name.chars().next().unwrap(); // 'R'
name.chars().next()는 Option<char>를 반환

"Rust"는 비어 있지 않으므로 Some('R')

.unwrap()은 'R'을 꺼냄

⚠️ 위험 요소
let empty = "";
let first = empty.chars().next().unwrap(); // ❌ panic 발생!
빈 문자열에서는 chars().next()가 None을 반환

.unwrap()은 None을 만나면 panic을 일으킴

즉, 프로그램이 강제 종료됨

🎯 실전 코드 분석

let starts_with = name
    .to_uppercase()
    .chars()
    .next()
    .unwrap();
name.to_uppercase() → String

.chars() → 문자 반복자

.next() → 첫 번째 문자 (Option<char>)

.unwrap() → 그 문자를 꺼냄 (char)

📌 이 코드는 name이 빈 문자열이 아니어야 안전하게 작동합니다.

✅ 안전한 대안

let starts_with = name
    .to_uppercase()
    .chars()
    .next()
    .unwrap_or('_'); // 기본값 지정
또는:

match name.to_uppercase().chars().next() {
    Some(c) if c == 'R' => format!("{} plays banjo", name),
    _ => format!("{} does not play banjo", name),
}
이렇게 하면 panic 없이 안전하게 처리할 수 있어요.

🧠 요약
메서드	설명
.unwrap()	Option 또는 Result에서 값을 꺼냄
성공 시	Some(value) → value 반환
실패 시	None 또는 Err → panic 발생
대안	.unwrap_or(), .unwrap_or_else(), match 구문
--------------------------------------------------------------------------
🔍 .get() 메서드란?
.get()는 Rust의 문자열(&str)이나 컬렉션(Vec, HashMap 등)에서 
범위 또는 인덱스를 기반으로 값을 안전하게 가져오는 메서드입니다.

반환값은 항상 Option<T> 타입

Some(value) → 범위가 유효할 때
None → 범위가 잘못되었거나 초과했을 때

📦 예시: 문자열에서 슬라이스 추출

let s = "Rust";
let first = s.get(..1); // Some("R")
..1은 시작부터 인덱스 1 전까지 (즉, 0번째 문자만)

s[0..1]과 같지만, get(..1)은 panic 없이 안전하게 처리

✅ 왜 .get()을 쓰나요?
Rust에서 문자열은 UTF-8로 인코딩되어 있어서, 
인덱싱이 복잡할 수 있어요. 예를 들어:

let s = "안녕";
let c = &s[0..1]; // ❌ panic 발생 가능
"안"은 UTF-8에서 3바이트

0..1은 유효한 문자 경계가 아니므로 panic 발생

.get()은 이런 위험을 피하기 위해 사용됩니다:

let c = s.get(0..1); // None → 안전하게 처리 가능
🎯 실전 코드 분석

match name.get(..1) {
    Some("r" | "R") => format!("{} plays banjo", name),
    _ => format!("{} does not play banjo", name)
}
name.get(..1)은 문자열의 첫 글자를 슬라이스로 추출 (&str)

"r" 또는 "R"과 비교
대소문자 구분 없이 banjo 여부 판단

📌 이 방식은 name이 빈 문자열일 경우에도 panic 없이 None을 반환하므로 안전합니다.

🧠 요약
메서드	설명
.get(range)	범위 기반 슬라이스를 안전하게 추출
반환값	Option<&str>
장점	UTF-8 경계 오류 없이 안전하게 처리
사용 예	name.get(..1) → 첫 글자 추출

*/