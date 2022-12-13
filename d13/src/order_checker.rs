pub fn is_in_write_order(left: &str, right: &str) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::is_in_write_order;

    #[test]
    fn from_AoC_string_and_shortest_path_test() {
        assert!(is_in_write_order("[1,1,3,1,1]", "[1,1,5,1,1]"));
        assert!(is_in_write_order("[[1],[2,3,4]]", "[[1],4]"));
        assert!(is_in_write_order("[9]", "[[8,7,6]]") == false);
        assert!(is_in_write_order("[[4,4],4,4]", "[[4,4],4,4,4]"));
        assert!(is_in_write_order("[7,7,7,7]", "[7,7,7]") == false);
        assert!(is_in_write_order("[]", "[3]"));
        assert!(is_in_write_order("[[[]]]", "[[]]") == false);
        assert!(
            is_in_write_order("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]")
                == false
        );
    }
}
