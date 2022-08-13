/// Recursive  Binary Search

fn binary_search(mut arr: Vec<u32>, find: u32) -> bool {
    arr.sort();

    if arr.is_empty(){
        return false
    };

    let high = arr.len();
    let mid = high / 2;
    let guess = arr[mid];

    if guess == find {
        return true;
    };

    return if guess < find {
        binary_search(arr[mid + 1..high].to_vec(), find)
    } else {
        binary_search(arr[0..(mid - 1)].to_vec(), find)
    };
}

#[cfg(test)]
mod tests {
    use crate::algorithms::binary_search_recursion::binary_search;

    #[test]
    fn test_binary_search() {
        let actual = binary_search(vec![48, 23, 43, 2, 3, 6, 10, 1, 33, 91, 87], 33);
        assert_eq!(true, actual);

        let actual = binary_search(vec![4, 6, 29, 64, 2, 3, 8, 0, 4], 22);
        assert_eq!(false, actual);

        let actual = binary_search(vec![], 30);
        assert_eq!(false, actual);

        let actual = binary_search(vec![2], 2);
        assert_eq!(true, actual);
    }
}