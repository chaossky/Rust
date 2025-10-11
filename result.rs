fn do_something_that_might_fail(i:i32)->Result<i32,String>{
    if i==13{
        Ok(13)
    }else{
        Err(String::from("맞는 숫자가 아닙니다."))
    }
}

fn main(){
    let result=do_something_that_might_fail(13);

    match result{
        Ok(v)=>println!("{} 발견",v),
        Err(e)=>println!("오류 : {}",e),            }
}
