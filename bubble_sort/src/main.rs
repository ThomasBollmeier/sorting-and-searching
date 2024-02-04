use bubble_sort::{bubble_sort, check_sorted, get_i32, make_random_vec, print_vec};

fn main() {
    let num_items = get_i32("Number of items: ");
    let max = get_i32("Maximum value: ");
    let mut numbers = make_random_vec(num_items, max);

    bubble_sort(&mut numbers);
    print_vec(&numbers, 20);

    if check_sorted(&numbers) {
        println!("The vector is sorted.");
    } else {
        println!("The vector is not sorted.");
    }
}
