/// Selection Sort
///
///
/// Ascending Order: Sorts an array by repeatedly finding the smallest ement in a vector of
/// elements
///
/// Descending Order: Sorts an array by repeatedly finding the biggest element in a vector of
/// elements
///
/// Time Complexity - O(n^2).
/// Reason - there are two nested object to iterate through
///
///
///
pub fn selection_sort(mut arr: Vec<u32>) -> Vec<u32> {
    let mut sorted_array = vec![];
    let len = arr.len();

    // return empty vec if empty array is given to us
    if len < 1 {
        return vec![];
    }

    for _ in 0..arr.len() {
        if arr.len() >= 1 {
            // find the index of the smallest element in the array
            let smallest = get_smallest(&arr);
            // insert the smallest element into our new sorted_array variable
            sorted_array.push(arr[smallest]);
            // finally remove the smallest array from our original array using the it's index
            arr.remove(smallest);
        }
    }

    sorted_array
}

/// get the smallest element in the array
pub fn get_smallest(arr: &Vec<u32>) -> usize {
    let mut smallest_idx = 0;
    let mut smallest = arr[0];

    for i in 0..arr.len() {
        if arr[i] < smallest {
            smallest = arr[i];
            smallest_idx = i;
        }
    }

    smallest_idx
}

#[cfg(test)]
mod tests {
    use crate::algorithms::selection_sort::{get_smallest, selection_sort};

    #[test]
    fn selection_sort_test() {
        let items = vec![9, 4, 6, 1, 5, 7, 3, 2, 8, 10];
        let empty: Vec<u32> = vec![];
        assert_eq!(3, get_smallest(&items));
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], selection_sort(items));
        assert_eq!(empty, selection_sort(vec![]));
    }
}
