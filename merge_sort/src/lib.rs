pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn merge_sort(mut arr: Vec<usize>, p: usize, r: usize) -> Vec<usize> {
    if p >= r {
        return arr;
    }

    let q = (p + r) / 2;
    merge_sort(arr, p, q);
    merge_sort(arr, q + 1, r);

    merge(arr, p, q, r);

    arr
}

fn merge(mut arr: Vec<usize>, p: usize, q: usize, r: usize) -> Vec<usize> {
    let left_length = q - p + 1;
    let right_length = r - q;
    
    let mut left_array = vec![0; left_length];
    let mut right_array = vec![0; right_length];

    for i in 0..left_length {
        left_array[i] = arr[p+i];
    }

    for i in 0..right_length {
        left_array[i] = arr[q+i];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    while i < left_length && j < right_length {
        if left_array[i] <= right_array[j] {
            arr[k] = left_array[i];
            i = i + 1;
        } else {
            arr[k] = left_array[j];
            j = j + 1;
        }
        k = k + 1;
    }

    while i < left_length {
        arr[k] = left_array[i];
        i = i + 1;
        k = k + 1;
    }

    while j < right_length {
        arr[k] = right_array[i];
        j = j + 1;
        k = k + 1;
    }

    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
