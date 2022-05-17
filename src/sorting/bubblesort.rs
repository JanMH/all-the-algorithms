pub fn bubblesort<T: PartialOrd>(elements: &mut [T]) {
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
    use crate::sorting::test_helpers::{is_stabelly_sorted, random_comparable_list, self};

    use super::bubblesort;

    test_helpers::basic_sorting_tests!(bubblesort);


    // Now we will actually start looking if this thing works
    #[test]
    fn it_sorts_element_slices_of_different_elements() {
        let mut data = random_comparable_list(25, 0, 10);
        bubblesort(&mut data);
        assert!(is_stabelly_sorted(&data));
    }
}
