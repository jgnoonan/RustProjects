/*
Reader and Writer types (inputs and outputs in Rust)

Rust's standard library features for input and output are organized
around two traits:  Read Write

1.  Read
Methods for byte-oriented input: readers
Stdin, File

2.  Write
Supports both byte-oriented and UTF-8 text output: writers
Stdout, File

*/

use std::fs::File;
use std::io::{Error, Read, BufReader}; // BufReader provides a buffering component for reading

fn main() -> Result <(), Error> {
    //Write
    let file_name = "cat.txt";
    match File::create(file_name) {
        Ok(file) => println!("{:?}", file),
        Err(_) => println!("unable to create the file {}", file_name)
    }
    match File::open(file_name) {
        Ok(file) => println!("{:?}", file),
        Err(_) => println!("unable to open the file {}", file_name)
    }

    // Read 
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    reader.read_to_string(&mut contents)?;
    println!("This is the contants..{}", contents);
    Ok(())
}
