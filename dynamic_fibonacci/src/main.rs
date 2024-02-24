use util::get_i64;

fn fibonacci_on_the_fly(values: &mut Vec<i64>, n: i64) -> i64 {
    if values.is_empty() {
        values.push(0);
        values.push(1);
    }
    let size = values.len();
    if n < size as i64 {
        return values[n as usize];
    }
    let result = fibonacci_on_the_fly(values, n - 2) + fibonacci_on_the_fly(values, n - 1);
    values.push(result);
    result
}

fn prefill_vector() -> Vec<i64> {
    let mut result = vec![0, 1];
    for i in 2..=92 {
        result.push(result[i - 2] + result[i - 1]);
    }
    result
}

fn fibonacci_bottom_up(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    let mut fib_i_minus_2 = 0i64;
    let mut fib_i_minus_1 = 1i64;
    let mut fib_i = fib_i_minus_1 + fib_i_minus_2;
    for _ in 1i64..n {
        // Calculate this value of fib_i.
        fib_i = fib_i_minus_1 + fib_i_minus_2;

        // Set fib_i_minus_2 and fib_i_minus_1 for the next value.
        fib_i_minus_2 = fib_i_minus_1;
        fib_i_minus_1 = fib_i;
    }
    return fib_i;
}

fn main() {
    // Initialize the prefilled vector.
    let prefilled_values = prefill_vector();

    // Create a vector for fill-on-the-fly.
    let mut fill_on_the_fly_values: Vec<i64> = vec![0, 1];

    loop {
        // Prompt the user for n.
        let n = get_i64("N: ");

        if n < 0 {
            break;
        }

        // Calculate the Fibonacci number.
        println!("Prefilled:  {}", prefilled_values[n as usize]);
        println!(
            "On the fly: {}",
            fibonacci_on_the_fly(&mut fill_on_the_fly_values, n)
        );
        println!("Bottom up:  {}", fibonacci_bottom_up(n));
        println!();
    }
}
