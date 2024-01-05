use std::cmp::Ordering;

pub fn bin_search(list: &[u8], item: u8) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        let mid = low + high;
        let guess = list[mid];

        match guess.cmp(&item) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
            Ordering::Equal => return Some(mid),
        };
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_should_be_found() {
        let list = [
            1, 10, 20, 47, 59, 63, 75, 88, 99, 107, 120, 133, 155, 162, 176, 188, 199, 200, 210,
            222,
        ];
        let expected_index = 10;

        assert_eq!(expected_index, bin_search(&list, 120).unwrap())
    }

    #[test]
    fn should_return_none_on_missing_value() {
        let list = [
            1, 10, 20, 47, 59, 63, 75, 88, 99, 107, 120, 133, 155, 162, 176, 188, 199, 200, 210,
            222,
        ];

        assert_eq!(None, bin_search(&list, 255))
    }
}
