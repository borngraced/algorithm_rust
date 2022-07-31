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
///
///
#[allow(unused)]
struct SelectionSort<T> {
    arr: Vec<T>,
    sorted: Vec<T>,
}

#[allow(unused)]
impl<T: Eq + Ord + Copy> SelectionSort<T> {
    pub fn new(arr: Vec<T>) -> Self {
        Self {
            arr,
            sorted: vec![],
        }
    }

    pub fn sort_ascending(&mut self) {
        let len = *&self.arr.len();

        // return empty vec if empty array is given to us
        if len < 1 {
            *self.sorted.as_mut() = vec![];
        }

        for _ in 0..len {
            if len >= 1 {
                // find the index of the smallest element in the array
                let smallest_idx = get_smallest_idx(*&&self.arr);
                // insert the smallest element into our new sorted_array variable
                self.sorted.push(*&self.arr[smallest_idx]);
                // finally remove the smallest array from our original array using the it's index
                self.arr.remove(smallest_idx);
            }
        }
    }

    pub fn sort_descending(&mut self) -> Vec<T> {
        let mut sorted_array = vec![];
        let len = self.arr.len();

        // return empty vec if empty array is given to us
        if len < 1 {
            return vec![];
        }

        for _ in 0..len {
            if len >= 1 {
                // find the index of the smallest element in the array
                let smallest_idx = get_smallest_idx(*&&self.arr);
                // insert the smallest element into our new sorted_array variable
                sorted_array.push(*&self.arr[smallest_idx]);
                // finally remove the smallest array from our original array using the it's index
                *&self.arr.remove(smallest_idx);
            }
        }

        sorted_array
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

#[cfg(test)]
mod tests {
    use crate::algorithms::selection_sort::{get_smallest_idx, SelectionSort};

    #[test]
    fn selection_sort_test() {
        let items = vec![9, 4, 6, 1, 5, 7, 3, 2, 8, 10];
        let empty: Vec<u32> = vec![];

        let mut sort = SelectionSort::new(items);
        sort.sort_ascending();
        assert_eq!(3, get_smallest_idx(&sort.arr));
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], sort.arr);

        let mut sort = SelectionSort::new(vec![]);
        sort.sort_ascending();
        assert_eq!(empty, sort.arr);
    }
}
