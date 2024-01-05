pub fn quick_sort(mut list: Vec<u32>) -> Vec<u32> {
    if list.len() < 2 {
        return list;
    }

    let pivot = list[0];
    list.remove(0);
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for elm in list.iter() {
        if elm < &pivot {
            left.push(*elm)
        } else if elm > &pivot {
            right.push(*elm)
        }
    }

    let sorted_left = quick_sort(left);
    let pivot_vec: Vec<u32> = vec![pivot];
    let sorted_right = quick_sort(right);

    return [
        sorted_left.as_slice(),
        pivot_vec.as_slice(),
        sorted_right.as_slice(),
    ]
    .concat();
}

mod tests {
    use super::*;

    #[test]
    fn should_sort_array() {
        let list: Vec<u32> = vec![21, 35, 19, 44, 41, 0, 22, 134, 13, 255, 33, 7, 4, 3, 1];
        let expected_list: Vec<u32> = vec![0, 1, 3, 4, 7, 13, 19, 21, 22, 33, 35, 41, 44, 134, 255];
        let sorted = quick_sort(list);

        for (index, item) in sorted.iter().enumerate() {
            assert_eq!(expected_list[index], *item);
        }
    }
}
