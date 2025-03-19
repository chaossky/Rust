// make a rust code that add all numbers in a text file
// each line is a number,
// make a vector of numbers,
// print the vector,
// and print the sum.

use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()>
{
    let mut numbers = Vec::new();
    let mut sum = 0.0;

    let file = File::open("numbers.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let number = line.unwrap().parse::<f32>().unwrap();
        numbers.push(number);
        sum += number;
    }

    println!("Numbers: {:?}", numbers);
    println!("Sum: {}", sum);

    Ok(())          
}

