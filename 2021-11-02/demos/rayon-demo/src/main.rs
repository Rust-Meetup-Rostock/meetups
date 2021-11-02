mod parallel;
mod processing_data;
mod serial;
mod utils;

use processing_data::ProcessingData;

fn main() {
    println!("Started data generation...");
    println!();

    let data = ProcessingData::default();

    println!();
    println!("finished data generation...");

    serial::run(data.clone());
    parallel::run(data);
}
