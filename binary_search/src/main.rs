use util::get_i32;
use quicksort::quicksort;

// Return the target's location in the vector and the number of tests.
// If the item is not found, return -1 and the number of tests.
fn binary_search(vec: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut num_tests = 0;
    let mut low = 0;
    let mut high = vec.len() - 1;

    while low <= high {
        num_tests += 1;
        let mid = (low + high) / 2;
        let mid_val = vec[mid];

        if mid_val < target {
            low = mid + 1;
        } else if mid_val > target {
            if mid == 0 {
                return (-1, num_tests);
            }
            high = mid - 1;
        } else {
            return (mid as i32, num_tests);
        }
    }

    (-1, num_tests)
}

fn main() {
    let num_items = get_i32("Items: ");
    let max_value = get_i32("Max: ");
    let mut target;

    let mut numbers = util::make_random_vec(num_items, max_value);
    quicksort(&mut numbers);
    util::print_vec(&numbers, 40);
    println!();

    loop {
        target = get_i32("Target (-1 to quit): ");
        if target == -1 {
            break;
        }

        let (idx, num_tests) = binary_search(&numbers, target);
        let message = if idx == -1 {
            format!("Target {} not found, {} tests", target, num_tests)
        } else {
            format!("numbers[{}] = {}, {} tests", idx, target, num_tests)
        };

        println!("{}", message);
        println!();
    }
}
