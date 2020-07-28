
pub fn quick_sort(vector: Vec<i32>, low: usize, high: usize) -> Vec<i32>{
    let mut  vector_copy = vector.to_vec();
    if low < high {
        let p = partition(vector_copy, low, high);
        
        vector_copy = quick_sort(p.1.to_vec(), low, p.0.saturating_sub(1));
        vector_copy = quick_sort(vector_copy.to_vec(), p.0 + 1, high);
    }
    return vector_copy.to_vec();
}

fn partition(vector: Vec<i32>, low: usize, high: usize) -> (usize, Vec<i32>) {
    let mut vector_copy = vector.to_vec();
    let pivot = vector_copy[high];
    let mut i: usize = low;
    for n in low..=high {
        let val = vector_copy[n];
        if val < pivot {
            vector_copy = swap(vector_copy.to_vec(), i, n);
            i = i + 1;
        }
    }
    vector_copy = swap(vector_copy.to_vec(), i, high);
    return (i, vector_copy);
}

fn swap(vector: Vec<i32>, i: usize, j: usize) -> Vec<i32> {
    let mut vector_copy = vector.to_vec();
    let temp = vector_copy[i];
    vector_copy[i] = vector[j];
    vector_copy[j] = temp;
    return vector_copy;
}
