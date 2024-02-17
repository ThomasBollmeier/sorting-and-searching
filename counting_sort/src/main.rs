use std::fmt;
use util::{get_i32, print_vec, Prng};

#[derive(Clone)]
struct Customer {
    id: String,
    num_purchases: i32,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.num_purchases)
    }
}

// Make a vector of random customers with num_purchases in the range [0 and max).
fn make_random_vec(num_items: i32, max_num_purchases: i32) -> Vec<Customer> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<Customer> = Vec::with_capacity(num_items as usize);
    for i in 0..num_items {
        let customer = Customer {
            id: format!("C{}", i),
            num_purchases: prng.next_i32(0, max_num_purchases),
        };
        vec.push(customer);
    }
    vec
}

fn check_sorted(vec: &Vec<Customer>) -> bool {
    for i in 1..vec.len() {
        if vec[i - 1].num_purchases > vec[i].num_purchases {
            return false;
        }
    }
    true
}

fn counting_sort(customers: &Vec<Customer>, max_num_purchases: i32) -> Vec<Customer> {
    let mut counts = vec![0; max_num_purchases as usize];
    let mut sorted = vec![
        Customer {
            id: "".to_string(),
            num_purchases: 0,
        };
        customers.len()
    ];

    // Count the number of customers with each number of purchases.
    for customer in customers {
        counts[customer.num_purchases as usize] += 1;
    }

    // Accumulate the counts.
    for i in 1..counts.len() {
        counts[i] += counts[i - 1];
    }

    // Build the sorted vector.
    for customer in customers.iter().rev() {
        let index = counts[customer.num_purchases as usize] - 1;
        sorted[index] = customer.clone();
        counts[customer.num_purchases as usize] -= 1;
    }

    sorted
}

fn main() {
    let num_items = get_i32("Number of items: ");
    let max_num_purchases = get_i32("Maximum number of purchases: ");
    let customers = make_random_vec(num_items, max_num_purchases);

    let customers_sorted = counting_sort(&customers, max_num_purchases);

    print_vec(&customers_sorted, 20);

    let message = if check_sorted(&customers_sorted) {
        "The vector is sorted."
    } else {
        "The vector is not sorted."
    };

    println!("{}", message);
}
