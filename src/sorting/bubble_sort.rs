pub fn bubble_sort<T: PartialOrd>(elements: &mut [T]) {
    if elements.len() <= 1 {
        return;
    }
    let mut swapped = true;
    let mut n = elements.len() - 1;
    while swapped {
        swapped = false;
        for i in 0..n {
            if elements[i] > elements[i + 1] {
                let (left, right) = elements.split_at_mut(i + 1);
                std::mem::swap(&mut left[i], &mut right[0]);
                swapped = true;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::test_helpers::{is_stabelly_sorted, random_comparable_list};

    use super::bubble_sort;

    // the following tests are just to check for crashes or endless loops

    #[test]
    fn it_sorts_empty_slices() {
        let mut data: [i32; 0] = [];
        bubble_sort(&mut data);
    }

    #[test]
    fn it_sorts_one_element_slices() {
        let mut data = [1];
        bubble_sort(&mut data);
    }

    #[test]
    fn it_sorts_element_slices_of_same_elements() {
        let mut data = [1, 1, 1, 1, 1, 1, 1];
        bubble_sort(&mut data);
    }

    // Now we will actually start looking if this thing works
    #[test]
    fn it_sorts_element_slices_of_different_elements() {
        let mut data = random_comparable_list(25, 0, 10);
        bubble_sort(&mut data);
        assert!(is_stabelly_sorted(&data));
    }
}
