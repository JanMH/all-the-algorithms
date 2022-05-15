use rand::Rng;

extern crate rand;

/// This represents an item that secretly holds information about its initial position in the array.
/// During the sort operation, it will only compare the value held. Later after the algorithm has run,
/// the initial index can be used to identify if the value was sorted in a stable manner.
pub struct StableSortComparableItem {
    value: i64,
    initial_index: usize,
}

impl PartialEq for StableSortComparableItem {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for StableSortComparableItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.value.partial_cmp(&other.value) {
            Some(core::cmp::Ordering::Equal) => return Some(core::cmp::Ordering::Equal),
            ord => return ord,
        }
    }
}

/// Generates a random list of StableSortComparableItems with the correct initial index
pub fn random_comparable_list(len: usize, min: i64, max: i64) -> Vec<StableSortComparableItem> {
    let mut result = Vec::new();
    result.reserve(len);
    let mut generator = rand::thread_rng();
    
    for i in 0..len {
        result.push(StableSortComparableItem {
            value: generator.gen_range(min..=max),
            initial_index: i,
        });
    }
    result
}

/// this function allows you to check if a slice of StableSortComparableItems was actually sorted in a stable manner
pub fn is_stabelly_sorted(items: &[StableSortComparableItem]) -> bool {
    for i in 0..(items.len() - 1) {
        if items[i].value > items[i + 1].value {
            return false;
        }
        if items[i].value == items[i + 1].value
            && items[i].initial_index > items[i + 1].initial_index
        {
            return false;
        }
    }
    return true;
}
