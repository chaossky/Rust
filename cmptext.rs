use std::fs;

fn main() {
    let afile="./fizzbuzz_python.txt";
    let bfiel="./fizzbuzz_rust.txt";

    let astr=fs::read_to_string(afile).unwrap();
    let bstr=fs::read_to_string(bfiel).unwrap();

    let astr=astr.trim();
    let bstr=bstr.trim();

    if astr==bstr {
        println!("OK");
    } else {
        println!("NG");
    }
}