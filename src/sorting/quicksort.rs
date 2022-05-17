pub fn quicksort<T: PartialOrd + Clone>(elements: &mut [T]) {
    if elements.len() < 2 {
        return;
    }
    let (left_p, right_p) = partition(elements);
    quicksort(&mut elements[0..left_p]);
    quicksort(&mut elements[right_p..]);
}

/// Partitions elements into left_side, pivots, right_side
/// returns the index of the leftmost item still being a pivot and the index of the item right of the last pivot
fn partition<T: PartialOrd + Clone>(elements: &mut [T]) -> (usize, usize) {
    let pivot = elements[elements.len() / 2].clone();
    let mut left_index = usize::MAX;
    let mut right_index = elements.len();

    loop {
        loop {
            left_index = left_index.wrapping_add(1);
            if elements[left_index] >= pivot {
                break;
            }
        }

        loop {
            right_index -= 1;
            if elements[right_index] <= pivot {
                break;
            }
        }

        if left_index >= right_index {
            break;
        }

        unsafe { swap_elements_unsafe(elements, left_index, right_index) }
    }

    // find the two indexes of elements where the pivots begin and end 
    // when there are multiple copies of the pivot

    while left_index > 0  {
        left_index -=1;
        if elements[left_index] != pivot {
            left_index += 1;
            break;
        }
    }

    while right_index < elements.len() -1 && elements[right_index] == pivot {
        right_index += 1;
    }

    (left_index, right_index)
}

#[inline]
unsafe fn swap_elements_unsafe<T: PartialOrd>(elements: &mut [T], index1: usize, index2: usize) {
    assert!(index1 != index2);
    let left = &elements[index1] as *const T as *mut T;
    let right = &elements[index2] as *const T as *mut T;
    std::mem::swap(&mut *left, &mut *right);
}

#[cfg(test)]
mod tests {
    use crate::sorting::test_helpers::{is_sorted, test_unstable_sort, self};

    use super::quicksort;
    test_helpers::basic_sorting_tests!(quicksort);

    #[test]
    fn test_quicksort_with_two_items() {
        let mut items = [-1, 1];
        quicksort(&mut items);
        assert!(is_sorted(items.iter()));
    }

    #[test]
    fn test_quicksort_with_two_items_2() {
        let mut items = [1, -1];
        quicksort(&mut items);
        assert!(is_sorted(items.iter()));
    }

    #[test]
    fn test_almost_all_the_same() {
        let mut items = [1, 1,1,1,1,1,0];
        quicksort(&mut items);
        assert!(is_sorted(items.iter()));
    }
    #[test]
    fn test_almost_all_the_same_and_ordered() {
        let mut items = [0,1,1,1,1,1,1];
        quicksort(&mut items);
        assert!(is_sorted(items.iter()));
    }

    #[test]
    fn test_quicksort_with_random_items() {
        test_unstable_sort(&quicksort, 10, -1000, 1000);
    }

    #[test]
    fn test_quicksort_with_lots_of_double_random_items() {
        test_unstable_sort(&quicksort, 2000, -1000, 1000);
    }
}
