pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn merge(mut arr: Vec<usize>, p: usize, q: usize, r: usize) -> Vec<usize> {
    let left_length = q - p + 1;
    let right_length = r - q;
    
    let left_array = 
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
