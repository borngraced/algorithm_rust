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

#[allow(unused)]
struct SelectionSort<T> {
    arr: Vec<T>,
}

#[allow(unused)]
impl<T: Eq + Ord + Copy> SelectionSort<T> {
    pub fn new(arr: Vec<T>) -> Self {
        Self { arr }
    }

    pub fn sort_ascending(&mut self) {
        let len = *&self.arr.len();
        let mut sorted = vec![];

        // return empty vec if empty array is given to us
        if len < 1 {
            return;
        } else {
            for _ in 0..len {
                if len >= 1 {
                    // find the index of the smallest element in the array
                    let smallest_idx = get_smallest_idx(*&&self.arr);
                    // insert the smallest element into our new sorted_array variable
                    sorted.push(*&self.arr[smallest_idx]);
                    // finally remove the smallest array from our original array using the it's index
                    self.arr.remove(smallest_idx);
                }
            }
        }

        self.arr = sorted;
    }

    pub fn sort_descending(&mut self) {
        let len = self.arr.len();
        let mut sorted = vec![];

        // return empty empty arr if empty array is given to us
        if len < 1 {
            return;
        } else {
            for _ in 0..len {
                if len >= 1 {
                    // find the index of the largest element in the array
                    let largest_idx = get_largest_idx(*&&self.arr);
                    // insert the largest element into our new sorted_array variable
                    sorted.push(*&self.arr[largest_idx]);
                    // finally remove the largest array from our original array using the it's index
                    *&self.arr.remove(largest_idx);
                }
            }
        }
        self.arr = sorted;
    }
}

/// get the smallest element in the array
pub fn get_smallest_idx<T: Eq + Ord + Copy>(arr: &Vec<T>) -> usize {
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

/// get the largest element in the array
pub fn get_largest_idx<T: Eq + Ord + Copy>(arr: &Vec<T>) -> usize {
    let mut largest_idx = 0;
    let mut largest = arr[0];

    for i in 0..arr.len() {
        if arr[i] > largest {
            largest = arr[i];
            largest_idx = i;
        }
    }

    largest_idx
}

#[cfg(test)]
mod tests {
    use crate::algorithms::selection_sort::{get_largest_idx, get_smallest_idx, SelectionSort};

    #[test]
    fn selection_sort_ascending_test() {
        let items = vec![9, 4, 6, 1, 5, 7, 3, 2, 8, 10];
        let empty: Vec<u32> = vec![];

        let mut sort = SelectionSort::new(items);
        sort.sort_ascending();
        assert_eq!(0, get_smallest_idx(&sort.arr));
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], sort.arr);

        let mut sort = SelectionSort::new(vec![]);
        sort.sort_ascending();
        assert_eq!(empty, sort.arr);
    }

    #[test]
    fn selection_sort_descending_test() {
        let items = vec![9, 4, 6, 1, 5, 7, 3, 2, 8, 10];
        let empty: Vec<u32> = vec![];

        let mut sort = SelectionSort::new(items);
        sort.sort_descending();
        assert_eq!(0, get_largest_idx(&sort.arr));
        assert_eq!(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], sort.arr);

        let mut sort = SelectionSort::new(vec![]);
        sort.sort_ascending();
        assert_eq!(empty, sort.arr);
    }
}
