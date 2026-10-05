const IDENTITY_AVOIDANCE_ATTEMPTS: usize = 20;

/// `random` yields values in `[0, 1)`.
pub fn shuffled<T: Clone>(items: &[T], mut random: impl FnMut() -> f64) -> Vec<T> {
    let mut copy: Vec<T> = items.to_vec();

    for index in (1..copy.len()).rev() {
        let swap_index: usize = ((random() * (index + 1) as f64) as usize).min(index);
        copy.swap(index, swap_index);
    }

    copy
}

pub fn shuffled_avoiding_identity<T: Clone + PartialEq>(items: &[T], mut random: impl FnMut() -> f64) -> Vec<T> {
    if items.len() < 2 {
        return items.to_vec();
    }

    for _ in 0..IDENTITY_AVOIDANCE_ATTEMPTS {
        let candidate: Vec<T> = shuffled(items, &mut random);

        if candidate != items {
            return candidate;
        }
    }

    items.iter().rev().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sequence_random(values: Vec<f64>) -> impl FnMut() -> f64 {
        let mut next_index: usize = 0;

        move || {
            let value: f64 = values[next_index % values.len()];
            next_index += 1;
            value
        }
    }

    #[test]
    fn shuffled_keeps_every_element() {
        let items: Vec<u32> = (0..20).collect();
        let mut result: Vec<u32> = shuffled(&items, sequence_random(vec![0.1, 0.9, 0.5, 0.3, 0.7]));

        result.sort();

        assert_eq!(result, items);
    }

    #[test]
    fn shuffled_avoiding_identity_differs_from_input_when_possible() {
        let items: Vec<u32> = vec![1, 2, 3, 4];
        let result: Vec<u32> = shuffled_avoiding_identity(&items, sequence_random(vec![0.999, 0.999, 0.999]));

        assert_ne!(result, items);
    }

    #[test]
    fn shuffled_avoiding_identity_returns_single_item_unchanged() {
        assert_eq!(shuffled_avoiding_identity(&[7], || 0.5), vec![7]);
    }
}
