pub fn quick_sort(vector: &mut [i32], low: usize, high: usize) -> &mut [i32]{
    let mut vector_copy = vector;
    if low < high {
        let p = partition(vector_copy, low, high);
        
        vector_copy = quick_sort(p.1, low, p.0.saturating_sub(1));
        vector_copy = quick_sort(vector_copy, p.0 + 1, high);
    }
    return vector_copy;
}

fn partition(vector: &mut [i32], low: usize, high: usize) -> (usize, &mut [i32]) {
    let mut vector_copy = vector;
    let pivot = vector_copy[high];
    let mut i: usize = low;
    for n in low..=high {
        let val = vector_copy[n];
        if val < pivot {
            vector_copy.swap(i, n);
            i = i + 1;
        }
    }
    vector_copy.swap(i, high);
    return (i, vector_copy);
}
