//이 함수는 문자열을 암호화 합니다.
// 문자열 리터럴과, 변환할 값을 매개변수로 받습니다.
// 그리고 스트링(문자열)을 Return(반환)합니다.
fn encrypt(text:&str, shift:i16)->String
{
    let code_a='A' as i16;//as는 강제적인 타입변환
    let code_z='Z' as i16;

    //결과를 반환할 변수를 지정한다.
    let mut result=String::new();

    //한글자씩 치환해서 암호화 한다.
    for ch in text.chars(){
        //문자 코드로 변환
        let mut code=ch as i16;
        if code_a<=code && code <=code_z{
            code=(code-code_a+shift+26)%26+code_a;
        }
        result.push(code as u8 as char)
        }
        return result
}

fn main()
{
    let enc=encrypt("I LOVE RUST",3);
    let dec=encrypt(&enc,-3);
    println!("{} => {}",enc,dec);
}