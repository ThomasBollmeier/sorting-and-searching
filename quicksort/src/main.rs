use util::{make_random_vec, make_vec_string};

fn partition(a: &mut [i32]) -> usize {
    let hi = a.len() - 1;
    let lo = 0;
    let pivot = a[hi];
    let mut i = 0;
    for j in lo..hi {
        if a[j] < pivot {
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, hi);
    i
}

fn quicksort(a: &mut [i32]) {
    if a.len() <= 1 {
        return;
    }
    let pivot_index = partition(a);
    quicksort(&mut a[0..pivot_index]);
    quicksort(&mut a[pivot_index + 1..]);
}

fn main() {
    let num_items = 1_000_000;
    let max_value = num_items * 10;
    let mut numbers = make_random_vec(num_items, max_value);

    println!("Unsorted: {}", make_vec_string(&numbers, 20));
    quicksort(&mut numbers);
    println!("Sorted: {}", make_vec_string(&numbers, 20));
}
