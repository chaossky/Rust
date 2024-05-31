fn main(){
    let mut a = 0;
    let mut b = 1;
    
    println!("{}",a);
    println!("{}",b);
    
    for i in 0..30{
        println!("f({})={}",i+2,a+b);
        let tmp=a;
        a = b;
        b = tmp+b;
    }
}