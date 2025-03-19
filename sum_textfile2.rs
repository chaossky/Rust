//make a rust program to sum the numbers in a text file 
// if a line is not a number,it will be ignored
// but if a line is a number,it will be added to the sum
// the sum will be printed to the console
// the text file will be read from the current directory
// the text file will be named textfile.txt
// the text file will be in the same directory as the program

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut sum = 0;
    let file = File::open("textfile.txt").unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.parse::<i32>().is_ok() {
            let num = line.parse::<i32>().unwrap();
            println!("The number is: {}", num);
            sum += num;
        } else {
            //sum += line.parse::<i32>().unwrap();
        }
    }
    println!("The sum is: {}", sum);
}