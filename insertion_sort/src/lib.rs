pub fn add(mut arr: Vec<usize> ) -> Vec<usize> {
    let length : usize = arr.len();
    
    for i in 1..length {
        let key = arr[i];
        
        let mut j : usize = i ;
        while j > 0 && arr[j-1] > key {
            arr[j] = arr[j-1];
            j = j - 1;
        }
        arr[j] = key;
    }

    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![5,2,7,1,9,4,3];
        let new_arr = add(arr);
        println!("{:?}", new_arr);
    }
}
