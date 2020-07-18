mod proj_1;
mod helpers;
use crate::proj_1::*;
use crate::helpers::cli_reader::*;

fn main() {
    triangle::check_triangle();
    UserInput::read_line("");
    let mut end = String::new();
    std::io::stdin()
        .read_line(&mut end)
        .expect("Failed to read");
}
