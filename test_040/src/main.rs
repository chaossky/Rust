fn do_something_that_might_fail(i:i32)->Result<f32,String> {
    if i==42{
        Ok(13.0)
    }else{
        Err(String::from("맞는 숫자가 아닙니다."))
    }
}

fn main()->Result<(),String> {
    let v=do_something_that_might_fail(42).unwrap();
    println!("{} 발견",v);

    let v=do_something_that_might_fail(1).unwrap();
    println!("{} 발견",v);

    Ok(())
}
