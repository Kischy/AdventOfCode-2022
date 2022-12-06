use std::collections::HashMap;

fn insert_or_increase(character: char, char_counters: &mut HashMap<char, u64>) {
    char_counters
        .entry(character)
        .and_modify(|c| *c += 1)
        .or_insert(1);
}

fn remove_or_decrease(character: char, char_counters: &mut HashMap<char, u64>) {
    if let Some(counter) = char_counters.get_mut(&character) {
        if *counter > 1 {
            *counter -= 1;
        } else {
            char_counters.remove(&character);
        }
    }
}

fn contains_only_unique(char_counters: &HashMap<char, u64>, num_of_unique_chars: usize) -> bool {
    char_counters.len() == num_of_unique_chars
}

pub fn find_marker_or_msg(datastream: &str, num_of_unique_chars: usize) -> usize {
    let mut char_counters: HashMap<char, u64> = HashMap::new();
    let data: Vec<char> = datastream.chars().collect();

    if data.len() < num_of_unique_chars {
        return 0;
    }

    for i in 0..num_of_unique_chars {
        insert_or_increase(data[i], &mut char_counters);
    }

    for i in num_of_unique_chars..data.len() {
        if contains_only_unique(&char_counters, num_of_unique_chars) {
            return i;
        }
        remove_or_decrease(data[i - num_of_unique_chars], &mut char_counters);
        insert_or_increase(data[i], &mut char_counters);
    }

    0
}

#[cfg(test)]
mod tests {
    use super::find_marker_or_msg;

    #[test]
    fn find_marker_tests_4_tests() {
        let unique_count = 4;
        assert_eq!(
            find_marker_or_msg("mjqjpqmgbljsphdztnvjfqwrcgsmlb", unique_count),
            7
        );
        assert_eq!(
            find_marker_or_msg("bvwbjplbgvbhsrlpgdmjqwftvncz", unique_count),
            5
        );
        assert_eq!(
            find_marker_or_msg("nppdvjthqldpwncqszvftbrmjlhg", unique_count),
            6
        );
        assert_eq!(
            find_marker_or_msg("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", unique_count),
            10
        );
        assert_eq!(
            find_marker_or_msg("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", unique_count),
            11
        );
    }

    #[test]
    fn find_marker_tests_14_tests() {
        let unique_count = 14;
        assert_eq!(
            find_marker_or_msg("mjqjpqmgbljsphdztnvjfqwrcgsmlb", unique_count),
            19
        );
        assert_eq!(
            find_marker_or_msg("bvwbjplbgvbhsrlpgdmjqwftvncz", unique_count),
            23
        );
        assert_eq!(
            find_marker_or_msg("nppdvjthqldpwncqszvftbrmjlhg", unique_count),
            23
        );
        assert_eq!(
            find_marker_or_msg("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", unique_count),
            29
        );
        assert_eq!(
            find_marker_or_msg("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", unique_count),
            26
        );
    }
}
