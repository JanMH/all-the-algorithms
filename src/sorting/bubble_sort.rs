pub fn bubble_sort<T: PartialOrd>(elements: &mut [T]) {


}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn it_sorts_empty_slices() {
        let mut data: Vec<i32> = Vec::new();
        bubble_sort(data.as_mut_slice());

    }
}
