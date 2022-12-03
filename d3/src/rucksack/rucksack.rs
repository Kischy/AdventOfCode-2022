pub(crate) fn get_wrongly_packed_item(rucksack: &str) -> char {
    let first_half = &rucksack[0..rucksack.len() / 2];
    let second_half = &rucksack[rucksack.len() / 2..];

    for c in first_half.chars() {
        if second_half.contains(c) {
            return c;
        }
    }

    '\0'
}

pub(crate) fn get_item_priority(item: char) -> u128 {
    let ascii_val = item as u128;
    if ascii_val >= 97 {
        return ascii_val - 97 + 1;
    }
    item as u128 - 65 + 27
}

#[cfg(test)]
mod tests {

    use crate::rucksack::rucksack::get_item_priority;
    use crate::rucksack::rucksack::get_wrongly_packed_item;
    #[test]
    fn get_wrongly_packed_item_tests() {
        assert_eq!(get_wrongly_packed_item("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
        assert_eq!(
            get_wrongly_packed_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            'L'
        );
        assert_eq!(get_wrongly_packed_item("PmmdzqPrVvPwwTWBwg"), 'P');
        assert_eq!(
            get_wrongly_packed_item("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            'v'
        );
        assert_eq!(get_wrongly_packed_item("ttgJtRGJQctTZtZT"), 't');
        assert_eq!(get_wrongly_packed_item("CrZsJsPPZsGzwwsLwLmpwMDw"), 's');
    }

    #[test]
    fn get_item_priority_tests() {
        assert_eq!(get_item_priority('A'), 27);
        assert_eq!(get_item_priority('C'), 29);
        assert_eq!(get_item_priority('Z'), 52);
        assert_eq!(get_item_priority('a'), 1);
        assert_eq!(get_item_priority('c'), 3);
        assert_eq!(get_item_priority('z'), 26);
    }
}
