// read each line in textfile called 'price.txt'
// and add them up and print the sum.

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("price.txt").expect("File not found");
    let reader = io::BufReader::new(file);
    let mut sum = 0;

    println!("Reading file...");
    for line in reader.lines() {
        let num: i32 = line.unwrap().parse().unwrap();
        sum += num;
    }
    println!("the sum is:{}",sum);
}