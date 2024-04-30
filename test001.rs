fn main(){
    let x=12;
    let a=12u8;
    let b=3.4;
    let c=4.5f32;
    let bv=true;
    let t=(12,false);
    let sentence="help me";

    println!("{} {} {} {} {} {} {} {}",
    x,a,b,c,bv,t.0,t.1,sentence);
}