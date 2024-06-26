mod employee_count;
mod segregate;
mod spiral;
mod wave_sort;

use employee_count::employee_count;
use segregate::segregate;
use spiral::{create_matrix, spiral};
use std::collections::{HashMap, LinkedList};
use std::time::Instant;
use wave_sort::wave_sort;

fn main() {
    let spiral1 = spiral(&[
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ]);
    print!("1. Spiral: ");
    spiral1.iter().for_each(|x| print!("{} ", x));
    println!();

    let giga_matrix = create_matrix(1000);

    let start = Instant::now();
    let giga_spiral = spiral(&giga_matrix);
    println!(
        "   Huge spiral length: {:?}, took {:?}. \n",
        giga_spiral.len(),
        start.elapsed()
    );

    let nums = [10, 5, 6, 3, 2, 20, 100, 80];
    let mut wave_sorted = nums;
    wave_sort(&mut wave_sorted);

    println!(
        "2. Wave sort:
    before: {:?},
     after: {:?}\n",
        nums, wave_sorted
    );

    let mut dict = HashMap::new();
    dict.insert("A", "C");
    dict.insert("B", "C");
    dict.insert("C", "F");
    dict.insert("D", "E");
    dict.insert("E", "F");
    dict.insert("F", "F");

    let result = employee_count(&dict);

    println!("3. Employee count:");
    result.iter().for_each(|(k, v)| {
        println!("  {}: {}", k, v);
    });

    let list = [1, 2, 2, 1, 2, 0, 2, 2];
    let mut linked_list = LinkedList::new();
    list.iter().for_each(|&x| linked_list.push_back(x));

    segregate(&mut linked_list);

    println!(
        "4. Segregate: {:?} -> {:?}\n",
        list,
        linked_list.iter().collect::<Vec<_>>()
    );

    let list = [1, 2, 2, 1, 2, 0, 2, 2];
    let mut linked_list = LinkedList::new();
    list.iter().for_each(|&x| linked_list.push_back(x));

    segregate(&mut linked_list);

    println!(
        "4. Segregate: {:?} -> {:?}\n",
        list,
        linked_list.iter().collect::<Vec<_>>()
    );
}
