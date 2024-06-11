fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    if denominator == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(numerator / denominator)
    }
}

fn main(){
    match divide(10.0, 2.0) {
        Ok(result) => println!("The result is {}", result),
        Err(e) => println!("Error: {}", e),
}
match divide(10,0.0){
    Ok(result) => println!("The result is {}", result),
    Err(e) => println!("Error: {}", e),

}
}