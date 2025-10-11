fn do_something_that_might_fail(i:i32)->Result<f32,String>{
    if i==42{
        Ok(42.0)
    }else{
        Err(String::from("맞는 숫자가 아닙니다."))
    }
}

fn main()->Result<(),String>{
    let result=do_something_that_might_fail(12);

    match result{
        Ok(v)=>println!("{} 발견",v),
        Err(_e)=>{
            return Err(String::from("main에서 뭔가 잘못 되었습니다!"));
        }
    }
    Ok(())
}