mod max_guests;

use max_guests::maximum_guests;

fn main() {
    let mut arrivals = vec![1, 2, 9, 5, 5];
    let mut exits = vec![4, 5, 12, 9, 12];
    println!(
        "maximum guests: {}",
        maximum_guests(&mut arrivals, &mut exits)
    );
}
