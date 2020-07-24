pub mod cli_reader;
pub mod error;

use rand::Rng;
use std::fs::File;
use std::io::Write;

pub fn generate_file(
    _file_path: String,
    size: isize,
    low: isize,
    high: isize,
) -> Result<(), error::Error> 
{
    if low >= high {
        return Err(error::Error::OutOfRange("".to_string()));
    }
    let mut file = File::create("foo.txt").expect("This has failed");
    for _ in 0..size {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(low, high);
        match writeln!(file, "{}", a) {
            _ => {}
        };
    }

    return Ok(());
}
