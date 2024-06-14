mod employee_count;
mod segregate;
mod spiral;
mod wave_sort;

use spiral::{create_matrix, spiral};
use std::time::Instant;
use wave_sort::wave_sort;

fn main() {
    let spiral1 = spiral(&[
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ]);
    print!("Spiral: ");
    spiral1.iter().for_each(|x| print!("{} ", x));
    println!();

    let giga_matrix = create_matrix(1000);

    let start = Instant::now();
    let giga_spiral = spiral(&giga_matrix);
    println!(
        "Huge spiral length: {:?}, took {:?}.",
        giga_spiral.len(),
        start.elapsed()
    );

    println!();
    let nums = [10, 5, 6, 3, 2, 20, 100, 80];
    let mut wave_sorted = nums;
    wave_sort(&mut wave_sorted);

    println!("wave sort before: {:?}, after: {:?}", nums, wave_sorted);
}
