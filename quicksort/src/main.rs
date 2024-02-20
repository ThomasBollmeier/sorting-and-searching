use quicksort::quicksort;
use util::{make_random_vec, make_vec_string};

fn main() {
    let num_items = 1_000_000;
    let max_value = num_items * 10;
    let mut numbers = make_random_vec(num_items, max_value);

    println!("Unsorted: {}", make_vec_string(&numbers, 20));
    quicksort(&mut numbers);
    println!("Sorted: {}", make_vec_string(&numbers, 20));
}
