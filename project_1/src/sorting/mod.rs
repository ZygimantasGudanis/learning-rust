use crate::helpers::error::Error;
use std::fs::File;
use std::io::{self, Write};
use io::BufRead;


pub mod quick_sort;

fn sort(vector: &mut [i32]) -> &mut [i32] {
    quick_sort::quick_sort(vector, 0, vector.len() - 1)
}

pub fn sort_file(input_path: String, output_path: String) -> Result<(), Error> {

    let file = File::open(input_path)?;
    
    let mut vector = Vec::new();
    
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        if let Ok(number) = line {
            vector.push(number.as_str().trim().parse::<i32>()?);
        }
    }
    println!("{:?}", vector);
    let sorted_vector = sort(vector.as_mut());
    println!("{:?}", sorted_vector);
    let mut file = File::create(output_path)?;
    for i in sorted_vector.iter(){
        writeln!(file, "{}", *i)?;
    }
    return Ok(());
}

