use util::get_i32;

// Return the target's location in the vector and the number of tests.
// If the item is not found, return -1 and the number of tests.
fn linear_search(vec: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut num_tests = 0;

    for (idx, v) in vec.iter().enumerate() {
        num_tests += 1;
        if *v == target {
            return (idx as i32, num_tests);
        }
    }

    (-1, num_tests)
}

fn main() {
    let num_items = get_i32("Items: ");
    let max_value = get_i32("Max: ");
    let mut target;

    let numbers = util::make_random_vec(num_items, max_value);
    util::print_vec(&numbers, 40);
    println!();

    loop {
        target = get_i32("Target (-1 to quit): ");
        if target == -1 {
            break;
        }

        let (idx, num_tests) = linear_search(&numbers, target);
        let message = if idx == -1 {
            format!("Target {} not found, {} tests", target, num_tests)
        } else {
            format!("numbers[{}] = {}, {} tests", idx, target, num_tests)
        };

        println!("{}", message);
        println!();
    }
}
