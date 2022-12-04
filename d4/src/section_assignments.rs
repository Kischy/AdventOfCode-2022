pub struct SectionRange {
    pub lower_boundary: u64,
    pub upper_boundary: u64,
}

impl SectionRange {
    pub fn from_string(range: &str) -> SectionRange {
        let vec = range.split("-").collect::<Vec<&str>>();
        SectionRange {
            lower_boundary: vec[0].parse::<u64>().unwrap(),
            upper_boundary: vec[1].parse::<u64>().unwrap(),
        }
    }
}

fn first_range_contains_second(first: &SectionRange, second: &SectionRange) -> bool {
    first.lower_boundary <= second.lower_boundary && first.upper_boundary >= second.upper_boundary
}

pub struct SectionAssignmentPair {
    pub first: SectionRange,
    pub second: SectionRange,
}

impl SectionAssignmentPair {
    pub fn from_string(range_pair: &str) -> SectionAssignmentPair {
        let vec = range_pair.split(",").collect::<Vec<&str>>();
        SectionAssignmentPair {
            first: SectionRange::from_string(vec[0]),
            second: SectionRange::from_string(vec[1]),
        }
    }

    pub fn full_range_is_overlapping(&self) -> bool {
        first_range_contains_second(&self.first, &self.second)
            || first_range_contains_second(&self.second, &self.first)
    }

    pub fn is_overlapping(&self) -> bool {
        self.first.lower_boundary <= self.second.upper_boundary
            && self.first.upper_boundary >= self.second.lower_boundary
    }
}

#[cfg(test)]
mod tests {
    use super::SectionAssignmentPair;
    use super::SectionRange;

    #[test]
    fn section_assignments_from_string_test() {
        let pair = SectionAssignmentPair::from_string("23-411,6-80");
        assert_eq!(pair.first.lower_boundary, 23);
        assert_eq!(pair.first.upper_boundary, 411);
        assert_eq!(pair.second.lower_boundary, 6);
        assert_eq!(pair.second.upper_boundary, 80);
    }

    #[test]
    fn full_range_is_overlapping_false_test() {
        let mut pair = SectionAssignmentPair {
            first: SectionRange {
                lower_boundary: 2,
                upper_boundary: 4,
            },
            second: SectionRange {
                lower_boundary: 6,
                upper_boundary: 8,
            }
        };
        pair.first.lower_boundary = 2;
        pair.first.upper_boundary = 4;
        pair.second.lower_boundary = 6;
        pair.second.upper_boundary = 8;
        assert!(pair.full_range_is_overlapping() == false);

        pair.first.lower_boundary = 2;
        pair.first.upper_boundary = 4;
        pair.second.lower_boundary = 1;
        pair.second.upper_boundary = 3;
        assert!(pair.full_range_is_overlapping() == false);
    }

    #[test]
    fn full_range_is_overlapping_true_tests() {
        let mut pair = SectionAssignmentPair {
            first: SectionRange {
                lower_boundary: 5,
                upper_boundary: 7,
            },
            second: SectionRange {
                lower_boundary: 4,
                upper_boundary: 9,
            }
        };
        assert!(pair.full_range_is_overlapping());

        pair.first.lower_boundary = 4;
        pair.first.upper_boundary = 6;
        pair.second.lower_boundary = 4;
        pair.second.upper_boundary = 9;
        assert!(pair.full_range_is_overlapping());

        pair.first.lower_boundary = 5;
        pair.first.upper_boundary = 9;
        pair.second.lower_boundary = 4;
        pair.second.upper_boundary = 9;
        assert!(pair.full_range_is_overlapping());

        pair.first.lower_boundary = 5;
        pair.first.upper_boundary = 9;
        pair.second.lower_boundary = 5;
        pair.second.upper_boundary = 9;
        assert!(pair.full_range_is_overlapping());
    }

    #[test]
    fn is_overlapping_true_tests() {
        let mut pair = SectionAssignmentPair {
            first: SectionRange {
                lower_boundary: 5,
                upper_boundary: 7,
            },
            second: SectionRange {
                lower_boundary: 7,
                upper_boundary: 9,
            }
        };
        assert!(pair.is_overlapping());

        pair.first.lower_boundary = 2;
        pair.first.upper_boundary = 8;
        pair.second.lower_boundary = 3;
        pair.second.upper_boundary = 7;
        assert!(pair.is_overlapping());

        pair.first.lower_boundary = 3;
        pair.first.upper_boundary = 4;
        pair.second.lower_boundary = 4;
        pair.second.upper_boundary = 6;
        assert!(pair.is_overlapping());

        pair.first.lower_boundary = 2;
        pair.first.upper_boundary = 6;
        pair.second.lower_boundary = 4;
        pair.second.upper_boundary = 8;
        assert!(pair.is_overlapping());
    }

    #[test]
    fn is_overlapping_false_tests() {
        let mut pair = SectionAssignmentPair {
            first: SectionRange {
                lower_boundary: 2,
                upper_boundary: 4,
            },
            second: SectionRange {
                lower_boundary: 6,
                upper_boundary: 8,
            }
        };       
        assert!(pair.is_overlapping() == false);

        pair.first.lower_boundary = 2;
        pair.first.upper_boundary = 3;
        pair.second.lower_boundary = 4;
        pair.second.upper_boundary = 5;
        assert!(pair.is_overlapping() == false);
    }
}
