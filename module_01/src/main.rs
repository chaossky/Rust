mod math_utils;
fn main() {
    let num=6.0;

    let squared=math_utils::square(num);
    println!("The square of {} is {}",num,squared);

    let cube=math_utils::cube(num);
    println!("The cube of {} is {}",num,cube);

    let square_root=math_utils::square_root(num);
    println!("The square root of {} is {}",num,square_root);

    let cube_root=math_utils::cube_root(num);
    println!("The cube root of {} is {}",num,cube_root);

    let factorial=math_utils::factorial(num);
    println!("The factorial of {} is {}",num,factorial);

    let a:f64=7.0;
    let b:f64=8.0;
    println!("The sum of {} and {} is {}",a,b,math_utils::sum(a,b)); 
    println!("The difference of {} and {} is {}",a,b,math_utils::sub(a,b)); 
    println!("The product of {} and {} is {}",a,b,math_utils::mul(a,b)); 
    println!("{} divied by {} is {}",a,b,math_utils::div(a,b)); 
    println!("{} divied by {} is {}",b,a,math_utils::div(b,a));
    println!("The remainder of {} and {} is {}",a,b,math_utils::mod_(a,b)); 
    println!("The remainder of {} and {} is {}",b,a,math_utils::mod_(b,a)); 
    println!("The power {} and {} is {}",a,b,math_utils::pow(a,b)); 
    println!("The power {} and {} is {}",b,a,math_utils::pow(b,a)); 
   // println!("The quotient of {} and {} is {}",b,a,math_utils::quotient(b,a)); 
}
