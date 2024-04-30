fn make_nothing()->(){
    return ();
}

fn make_nothing2(){
}

struct Marker;

fn main() {
    let a=make_nothing();
    let b=make_nothing2();
    let _m=Marker;

    println!("The value of a : {:?}",a);
    println!("The value of b : {:?}",b);
    println!("Hello, world!");

    let x=32;
    if x<32{
        println!("32보다 작다");
    }
    else if x==32{
        println!("32와 같다");
    }
    else{
        println!("32보다 크다");
    }
}
