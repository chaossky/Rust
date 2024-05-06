fn main() {
    let mut i32_vec=Vec::<i32>::new();
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    for i in i32_vec.iter() {
        println!("{}",i);
    }

    let mut float_vec=Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    for f in float_vec.iter() {
        println!("{}",f);
    }

   let string_vec=vec![String::from("Hello"),String::from("world")];

   for word in string_vec.iter() {
       println!("{}",word);
   }
}
