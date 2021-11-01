use crate::processing_data::ProcessingData;

use super::utils::*;
use rayon::prelude::*;
use std::ops::Range;
use std::time::Duration;
use std::time::Instant;

pub fn run(data: ProcessingData) {
    println!("Started parallel processing:");
    println!();

    println!("iter: {} ms", iter(data.range).as_millis());
    println!(
        "iter_mut: {} µs",
        iter_mut(data.numbers.clone()).as_micros()
    );
    println!(
        "iter_all: {} µs",
        iter_all(data.numbers.clone()).as_micros()
    );
    println!("iter_any(): {} µs", iter_any(data.numbers).as_micros());
    println!("sort(): {} µs", sort(data.strings).as_micros());

    println!();
    println!("Parallel processing finished...");
}

fn iter(range: Range<i32>) -> Duration {
    let instant = Instant::now();
    range.into_par_iter().for_each(|_| do_something());
    instant.elapsed()
}

fn iter_mut(mut source: Vec<i32>) -> Duration {
    let instant = Instant::now();
    source.par_iter_mut().for_each(|number| *number -= 1);
    instant.elapsed()
}

fn iter_all(source: Vec<i32>) -> Duration {
    let instant = Instant::now();
    let _result = source.into_par_iter().all(|number| is_odd(number));
    instant.elapsed()
}

fn iter_any(source: Vec<i32>) -> Duration {
    let instant = Instant::now();
    let _result = source.into_par_iter().any(|number| is_odd(number));
    instant.elapsed()
}

fn sort(mut source: Vec<String>) -> Duration {
    let instant = Instant::now();
    source.par_sort();
    instant.elapsed()
}
