mod proj_1;
mod proj_2;
mod helpers;
mod sorting;

use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Set speed
    #[structopt(short, long, default_value = "42")]
    speed: f64,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    // the long option will be translated by default to kebab case,
    // i.e. `--nb-cars`.
    /// Number of cars
    #[structopt(short = "c", long)]
    nb_cars: Option<i32>,

    /// admin_level to consider
    #[structopt(short, long)]
    level: Vec<String>,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}


fn main() {
    //let opt = Opt::from_args();
    // helpers::generate_file("D:\\Rust\\Projects\\project_1\\foo.txt".to_string(), 20, 0, 20).expect("failed");
    // sorting::sort_file("D:\\Rust\\Projects\\project_1\\foo.txt".to_string(), "D:\\Rust\\Projects\\project_1\\foo-sorted.txt".to_string()).expect("msg");
    // sorting::sort();
}
