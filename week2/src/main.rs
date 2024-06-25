mod heater_radius;
mod max_guests;
mod prefix_sum;
mod subarray_sums_equal_k;

use crate::prefix_sum::NumMatrix;
use crate::subarray_sums_equal_k::{subarray_sums, subarray_sums_2ptr};
use heater_radius::min_heater_radius;
use max_guests::maximum_guests;

fn main() {
    let mut arrivals = vec![1, 2, 9, 5, 5];
    let mut exits = vec![4, 5, 12, 9, 12];
    println!(
        "maximum guests: {}",
        maximum_guests(&mut arrivals, &mut exits)
    );

    let houses = vec![1, 2, 3, 4];
    let heaters = vec![1, 4];
    println!(
        "minimum heater radius: {}",
        min_heater_radius(&houses, &heaters)
    );

    let nums = vec![1, 0, 1, -1, 0, 1];
    let k = 2;
    println!("subarray sums: {}", subarray_sums(&nums, k));

    let nums = vec![1, 0, 1, 1, 0, 1];
    let k = 2;
    println!("subarray sums: {}", subarray_sums_2ptr(&nums, k));

    let matrix = NumMatrix::new(&[
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ]);

    println!("prefix sums: {}", matrix.sum_region(2, 1, 4, 3));
}
