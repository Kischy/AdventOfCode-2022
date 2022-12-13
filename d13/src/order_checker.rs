use std::cmp::Ordering;

fn vec_compare(left: &Vec<serde_json::Value>, right: &Vec<serde_json::Value>) -> Option<Ordering> {
    for (l, r) in left.iter().zip(right) {
        let res = value_compare(l, r);
        if res == Some(Ordering::Greater) || res == Some(Ordering::Less) {
            return res;
        }
    }

    left.len().partial_cmp(&right.len())
}

fn value_compare(left: &serde_json::Value, right: &serde_json::Value) -> Option<Ordering> {
    match (left, right) {
        (serde_json::Value::Number(l), serde_json::Value::Number(r)) => {
            let li = l.as_u64();
            let ri = r.as_u64();
            li.partial_cmp(&ri)
        }
        (serde_json::Value::Number(_l), serde_json::Value::Array(r)) => {
            vec_compare(&vec![left.clone()], r)
        }
        (serde_json::Value::Array(l), serde_json::Value::Number(_r)) => {
            vec_compare(l, &vec![right.clone()])
        }
        (serde_json::Value::Array(l), serde_json::Value::Array(r)) => vec_compare(l, r),
        _ => {
            panic!("Unkown combination")
        }
    }
}

pub fn is_in_correct_order(left: &str, right: &str) -> bool {
    let ljson: Option<serde_json::Value> = serde_json::from_str(left).ok();
    let rjson: Option<serde_json::Value> = serde_json::from_str(right).ok();

    match (ljson, rjson) {
        (None, None) => {
            println! {"Both?: {left}|||{right}"}
            return false;
        }
        (None, Some(_)) => {
            println! {"Left?: {left}|||{right}"};
            return false;
        }
        (Some(_), None) => {
            println! {"Right?: {left}|||{right}"};
            return false;
        }
        (Some(lj), Some(rj)) => {
            let ordopt = value_compare(&lj, &rj);

            if let Some(ord) = ordopt {
                match ord {
                    Ordering::Less => return true,
                    Ordering::Equal => return true,
                    Ordering::Greater => return false,
                }
            }

            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::is_in_correct_order;

    #[test]
    fn is_in_correct_order_tests() {
        assert!(is_in_correct_order("[1,1,3,1,1]", "[1,1,5,1,1]"));
        assert!(is_in_correct_order("[[1],[2,3,4]]", "[[1],4]"));
        assert!(is_in_correct_order("[9]", "[[8,7,6]]") == false);
        assert!(is_in_correct_order("[[4,4],4,4]", "[[4,4],4,4,4]"));
        assert!(is_in_correct_order("[7,7,7,7]", "[7,7,7]") == false);
        assert!(is_in_correct_order("[]", "[3]"));
        assert!(is_in_correct_order("[[[]]]", "[[]]") == false);
        assert!(
            is_in_correct_order("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]")
                == false
        );
        assert!(is_in_correct_order(
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]"
        ));
    }
}
