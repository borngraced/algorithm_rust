/// BINARY SEARCH: an efficient algorithms for finding an element in an array from a sorted list of
/// items
///
/// Time Complexity: O(logN)
/// AUxiliary Space: O(1)
///
///
///Binary search alogrithm can be performed in Iteratively and Recursively
///

/// Iterative Binary Search

#[allow(unused)]
fn binary_search(mut arr: Vec<u32>, find: u32) -> bool {
    // let's return false anyways if array is empty.
    if arr.is_empty() {
        return false;
    };

    // let perform quick check is an array has only 1 element.
    if arr.len() <= 1 {
        return arr[0] == find;
    };

    // if the above terms are not met then we'd proceed with our algorithm.
    // We need to sort the array first hence why we do arr.sort().
    arr.sort();

    let mut low = 0;
    let mut high = arr.len() - 1;

    // we need to keep our loop on while low is less than or not equal high.
    while low <= high {
        // the median value idx of our arr
        let mid = (low + high) / 2;
        // let's say we guess the median value of the be the suppose value we're finding here.
        let guess = arr[mid];

        // if our guess if equal to the value we need to find then we've succesfuuly gotten to the end
        // of our algorithm.
        if guess == find {
            return true;
        };
        // hence, if guess is lesser than find then we need to shift our low to be the median value index (mid) + 1 and if otherwise we need to shift our high to median value index (mid) - 1.
        if guess < find {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::algorithms::binary_search::binary_search;

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
