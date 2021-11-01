use rand::distributions::Alphanumeric;
use rand::Rng;
use rayon::prelude::*;
use std::thread;
use std::time;

pub fn get_random_int_vec(size: usize) -> Vec<i32> {
    let mut generator = rand::thread_rng();

    (0..=size)
        .into_iter()
        .map(|_| generator.gen::<i32>())
        .collect()
}

pub fn get_random_str_vec(size: usize) -> Vec<String> {
    let mut vec = vec![String::new(); size];
    vec.par_iter_mut().for_each(|p| {
        let generator = rand::thread_rng();
        *p = generator
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
    });

    vec
}

pub fn do_something() {
    let duration = time::Duration::from_secs(1);
    thread::sleep(duration);
}

pub fn is_odd(number: i32) -> bool {
    number % 2 != 0
}
