// Use bubble sort to sort the vector.
pub fn bubble_sort(vec: &mut Vec<i32>) {
    loop {
        let mut swapped = false;
        for i in 1..vec.len() {
            if vec[i - 1] > vec[i] {
                vec.swap(i - 1, i);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

pub fn check_sorted(vec: &Vec<i32>) -> bool {
    for i in 1..vec.len() {
        if vec[i - 1] > vec[i] {
            return false;
        }
    }
    return true;
}
