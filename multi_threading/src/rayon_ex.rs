extern crate rayon;

use std::{thread, time};
use rayon::prelude::*;

pub fn rayon_main() {
    let cpu_num = rayon::current_num_threads();
    println!("Rayon thread pool of size: {}", cpu_num);

    let input = &mut [1, 2, 3];
    println!("{:?}", input);
    let sum = sum_of_squares(input);
    println!("sum of sq {:?}", sum);

    increment_all(input);
    println!("{:?}", input);

    let sum = sum_of_squares(input);
    println!("sum of sq {:?}", sum);
}

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter().map(|&i| i * i).sum()
}

fn increment_all(input: &mut [i32]) {
    input.par_iter_mut().for_each(|p| *p += 1);

    let sleep_time = time::Duration::from_secs(1);
    thread::sleep(sleep_time);

}
