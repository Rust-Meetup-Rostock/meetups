mod parallel;
mod processing_data;
mod serial;
mod utils;

use processing_data::ProcessingData;

fn main() {
    let data = ProcessingData::default();

    serial::run(data.clone());
    parallel::run(data);
}
