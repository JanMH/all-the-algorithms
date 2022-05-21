#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GaleShapleyError {
    DifferentSetSize,
    InsufficientPreferences,
}

pub fn gale_shapley(
    a: &[Vec<usize>],
    b: &[Vec<usize>],
) -> Result<Vec<(usize, usize)>, GaleShapleyError> {
    if a.len() != b.len() {
        return Err(GaleShapleyError::DifferentSetSize);
    }

    let mut proposed_index: Vec<_> = std::iter::repeat(0).take(a.len()).collect();
    let mut matched_a: Vec<Option<usize>> = std::iter::repeat(None).take(a.len()).collect();
    let mut matched_b: Vec<Option<usize>> = std::iter::repeat(None).take(a.len()).collect();
    let mut free_a: Vec<_> = (0..a.len()).collect();

    while !free_a.is_empty() {
        let idx = *free_a.last().unwrap();
        let next_proposed = a[idx].get(proposed_index[idx]);
        proposed_index[idx] += 1;

        if let Some(next_proposed) = next_proposed {
            if is_better_match(b, *next_proposed, matched_b[*next_proposed], idx)? {
                free_a.pop();
                replace_match(
                    &mut free_a,
                    &mut matched_a,
                    &mut matched_b,
                    idx,
                    *next_proposed,
                );
            }
        } else {
            return Err(GaleShapleyError::InsufficientPreferences);
        }
    }

    println!("{:?}", matched_a);
    println!("{:?}", matched_b);

    let result = matched_a
        .iter()
        .zip(0..)
        .map(|(b, a)| (a, b.unwrap()))
        .collect();
    Ok(result)
}

fn replace_match(
    free_a: &mut Vec<usize>,
    matched_a: &mut Vec<Option<usize>>,
    matched_b: &mut Vec<Option<usize>>,
    index_a: usize,
    next_match: usize,
) {
    if let Some(mb) = matched_b[next_match] {
        matched_b[matched_a[mb].unwrap()] = None;
        matched_a[mb] = None;
        free_a.push(mb);
    }

    matched_b[next_match] = Some(index_a);
    matched_a[index_a] = Some(next_match);
}

fn is_better_match(
    b: &[Vec<usize>],
    b_index: usize,
    current_match: Option<usize>,
    next_match: usize,
) -> Result<bool, GaleShapleyError> {
    if current_match.is_none() {
        return Ok(true);
    }

    let current_match = current_match.unwrap();
    for preference in &b[b_index] {
        if *preference == current_match {
            return Ok(false);
        } else if *preference == next_match {
            return Ok(true);
        }
    }

    Err(GaleShapleyError::InsufficientPreferences)
}

#[cfg(test)]
mod test {
    extern crate rand;

    use rand::prelude::SliceRandom;
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_no_preferences() {
        let a = [vec![], vec![]];
        let b = [vec![0, 1], vec![1, 0]];
        assert_eq!(
            gale_shapley(&a, &b),
            Err(GaleShapleyError::InsufficientPreferences)
        );
    }

    #[test]
    fn test_different_set_sizes() {
        let a = [vec![]];
        let b = [vec![0, 1], vec![1, 0]];
        assert_eq!(
            gale_shapley(&a, &b),
            Err(GaleShapleyError::DifferentSetSize)
        );
    }
    #[test]
    fn test_empty_case() {
        let a = [];
        let b = [];
        assert_eq!(gale_shapley(&a, &b), Ok(vec![]));
    }

    #[test]
    fn test_trivial_case() {
        let a = [vec![0]];
        let b = [vec![0]];
        assert_eq!(gale_shapley(&a, &b), Ok(vec![(0, 0)]));
    }

    #[test]
    fn test_simple_case() {
        let a = [vec![1, 0], vec![0, 1]];
        let b = [vec![0], vec![0]];
        let result = gale_shapley(&a, &b);
        assert!(result.is_ok());
        let result: HashSet<_> = HashSet::from_iter(result.unwrap().into_iter());
        assert_eq!(result, HashSet::from([(1, 0), (0, 1)]));
    }

    #[test]
    fn test_stealing_test_case() {
        let a = [vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]];
        let b = [vec![0, 2, 1], vec![0, 1, 2], vec![2, 1, 0]];
        let result = gale_shapley(&a, &b);
        assert!(result.is_ok());
        let result: HashSet<_> = HashSet::from_iter(result.unwrap().into_iter());
        assert_eq!(result, HashSet::from([(0, 0), (1, 1), (2,2)]));
    }

    fn gen_preferences(num_elements: usize) -> Vec<usize> {
        let mut result: Vec<usize> = (0..num_elements).collect();
        result.shuffle(&mut rand::thread_rng());
        return result;
    }

    fn gen_preference_list(num_elements: usize) -> Vec<Vec<usize>> {
        (0..num_elements)
            .map(|_| gen_preferences(num_elements))
            .collect()
    }

    fn all_indeces_uniquely_assigned<'a>(a: impl Iterator<Item = &'a usize>) -> bool {
        let mut items: Vec<usize> = a.cloned().collect();
        items.sort();
        let indices: Vec<_> = (0..items.len()).collect();
        items == indices
    }

    fn is_stable_marriage(
        a: &[Vec<usize>],
        b: &[Vec<usize>],
        matches: &Result<Vec<(usize, usize)>, GaleShapleyError>,
    ) -> bool {
        if matches.is_err() {
            return false;
        }
        let matches = matches.as_ref().unwrap();

        if matches.len() != a.len() || a.len() != b.len() {
            return false;
        }

        assert!(all_indeces_uniquely_assigned(
            matches.iter().map(|(a, _)| a)
        ));
        assert!(all_indeces_uniquely_assigned(
            matches.iter().map(|(_, b)| b)
        ));

        let mut match_for_a: Vec<_> = std::iter::repeat(0).take(a.len()).collect();
        let mut match_for_b: Vec<_> = std::iter::repeat(0).take(a.len()).collect();

        for (a, b) in matches {
            match_for_a[*a] = *b;
            match_for_b[*b] = *a;
        }

        for i in 0..a.len() {
            for j in 0..b.len() {
                let pref_idx_a_match = a[i].iter().position(|x| *x == match_for_a[i]);
                let pref_idx_b_match = b[j].iter().position(|x| *x == match_for_b[j]);

                let pref_idx_a_item = a[i].iter().position(|x| *x == j);
                let pref_idx_b_item = b[j].iter().position(|x| *x == i);

                if pref_idx_a_item < pref_idx_a_match && pref_idx_b_item < pref_idx_b_match {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_fully_connected_cases() {
        for num_elements in 5..30 {
            let a = gen_preference_list(num_elements);
            let b = gen_preference_list(num_elements);
            let result = gale_shapley(&a, &b);

            assert!(is_stable_marriage(&a, &b, &result));
        }
    }
}
