use super::processing_data::ProcessingData;
use super::utils::*;
use std::ops::Range;
use std::time::Duration;
use std::time::Instant;

pub fn run(data: ProcessingData) {
    println!("Started serial processing:");
    println!();

    println!(
        "iter({}): {} ms",
        data.range.end.clone(),
        iter(data.range).as_millis()
    );
    println!(
        "iter_mut({}): {} µs",
        data.numbers.len(),
        iter_mut(data.numbers.clone()).as_micros()
    );
    println!(
        "iter_all({}): {} µs",
        data.numbers.len(),
        iter_all(data.numbers.clone()).as_micros()
    );
    println!(
        "iter_any({}): {} µs",
        data.numbers.len(),
        iter_any(data.numbers).as_micros()
    );
    println!(
        "sort({}): {} µs",
        data.strings.len(),
        sort(data.strings).as_micros()
    );

    println!();
    println!("Serial processing finished...");
}

fn iter(range: Range<i32>) -> Duration {
    let instant = Instant::now();
    range.for_each(|_| do_something());
    instant.elapsed()
}

fn iter_mut(mut source: Vec<i32>) -> Duration {
    let instant = Instant::now();
    source.iter_mut().for_each(|number| *number -= 1);
    instant.elapsed()
}

fn iter_any(source: Vec<i32>) -> Duration {
    let instant = Instant::now();
    let _result = source.into_iter().any(|number| is_odd(number));
    instant.elapsed()
}

fn iter_all(source: Vec<i32>) -> Duration {
    let instant = Instant::now();
    let _result = source.into_iter().all(|number| is_odd(number));
    instant.elapsed()
}

fn sort(mut source: Vec<String>) -> Duration {
    let instant = Instant::now();
    source.sort();
    instant.elapsed()
}
