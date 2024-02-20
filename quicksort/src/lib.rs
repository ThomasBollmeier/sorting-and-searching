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

pub fn quicksort(a: &mut [i32]) {
    if a.len() <= 1 {
        return;
    }
    let pivot_index = partition(a);
    quicksort(&mut a[0..pivot_index]);
    quicksort(&mut a[pivot_index + 1..]);
}
