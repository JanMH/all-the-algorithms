use std::collections::HashMap;
use std::hash::Hash;

pub fn brents_cycle_detection<T: PartialEq, F: Fn(&T) -> T>(f: &F, x0: T) -> (usize, usize) {
    // if we have a cycle at first element, return immediately to avoid moving x0 to tortoise
    if f(&x0) == x0 {
        return (1, 0);
    }

    let mut power = 1;
    let mut lambda = 1;
    let mut hare = f(&x0);
    // because of the check we did above we can initialize this to any value
    let mut tortoise = f(&x0);
    loop {
        if power == lambda {
            tortoise = hare;
            power *= 2;
            lambda = 1;
            hare = f(&tortoise);
        } else {
            hare = f(&hare);
            lambda += 1;
        }
        if tortoise == hare {
            break;
        }
    }

    let mut tortoise = x0;
    let mut hare = f(&tortoise);
    for _ in 0..lambda - 1 {
        // decrease the range by one because we already "added" one to the hare to avoid moving x0
        hare = f(&hare)
    }

    let mut mu = 0;
    while tortoise != hare {
        tortoise = f(&tortoise);
        hare = f(&hare);
        mu += 1
    }

    (lambda, mu)
}

pub fn hash_map_to_cyclic_function<T: Eq + Clone + Hash>(
    values: HashMap<T, T>,
) -> impl Fn(&Option<T>) -> Option<T> {
    move |xi: &Option<T>| {
        if let Some(value) = xi {
            let result = values.get(value)?;
            Some(result.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_initial_case() {
        let values = HashMap::from([(1, 1)]);
        assert_eq!(
            brents_cycle_detection(&hash_map_to_cyclic_function(values), Some(1)),
            (1, 0)
        )
    }

    #[test]
    fn test_small_case() {
        let values = HashMap::from([(1, 2), (2, 1)]);
        assert_eq!(
            brents_cycle_detection(&hash_map_to_cyclic_function(values), Some(1)),
            (2, 0)
        )
    }

    #[test]
    fn test_small_case2() {
        let values = HashMap::from([(1, 2), (2, 3), (3, 4), (4, 2)]);
        assert_eq!(
            brents_cycle_detection(&hash_map_to_cyclic_function(values), Some(1)),
            (3, 1)
        )
    }

    fn gen_test_hash(mu: usize, lambda: usize) -> HashMap<usize, usize> {
        let mut result = HashMap::new();
        for i in 0..mu {
            result.insert(i + 1, i + 2);
        }

        for i in mu..(mu + lambda - 1) {
            result.insert(i + 1, i + 2);
        }
        result.insert(mu + lambda, mu + 1);
        result
    }

    #[test]
    fn iterate_different_vals_test() {
        for lambda in 1..20 {
            for mu in 0..20 {
                let hashvalues = gen_test_hash(mu, lambda);
                let fun = hash_map_to_cyclic_function(hashvalues);
                assert_eq!(brents_cycle_detection(&fun, Some(1)), (lambda, mu));
            }
        }
    }
}
