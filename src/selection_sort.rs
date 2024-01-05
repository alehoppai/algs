use std::vec;

fn find_smallest(list: &Vec<u32>) -> usize {
    let mut value = list[0];
    let mut index = 0;

    for i in 1..list.len() {
        if list[i] < value {
            value = list[i];
            index = i;
        }
    }

    return index;
}

pub fn selection_sort(mut list: Vec<u32>) -> Vec<u32> {
    let mut sorted: Vec<u32> = Vec::with_capacity(list.capacity());

    for _ in 0..list.len() {
        let smallest_index = find_smallest(&list);
        sorted.push(list[smallest_index]);
        list.remove(smallest_index);
    }

    return sorted;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sort_array() {
        let list: Vec<u32> = vec![21, 35, 19, 44, 41, 0, 22, 134, 13, 255, 33, 7, 4, 3, 1];
        let expected_list: Vec<u32> = vec![0, 1, 3, 4, 7, 13, 19, 21, 22, 33, 35, 41, 44, 134, 255];
        let sorted = selection_sort(list);

        for (index, item) in sorted.iter().enumerate() {
            assert_eq!(expected_list[index], *item);
        }
    }
}
