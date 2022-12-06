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

pub fn find_marker(datastream: &str) -> usize {
    let num_of_unique_chars = 4;
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
    use super::find_marker;

    #[test]
    fn find_marker_tests() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
