use std::io::BufRead;
use std::str::FromStr;

pub fn item_distance(window: &[(usize, usize)]) -> usize {
    match window {
        [(a_idx, a_value), (b_idx, b_value)] => {
            if a_value + 1 == *b_value {
                std::cmp::max(b_idx, a_idx) - std::cmp::min(a_idx, b_idx)
            } else {
                0
            }
        }
        [_, ..] => 0,
        [] => 0,
    }
}

pub fn consecutive_distance(row: Vec<usize>) -> usize {
    let mut enumerated_row = row.into_iter().enumerate().collect::<Vec<(usize, usize)>>();
    enumerated_row.sort_by(|a, b| match (a, b) {
        ((_a_idx, a_val), (_b_idx, b_val)) => a_val.cmp(b_val),
    });

    // (0, 1) (2, 2) (6, 3) (1, 7) (4, 8) (3, 11) (5, 34)
    (&enumerated_row)
        .windows(2)
        .map(item_distance)
        .fold(0, |acc, i| acc + i)
}

pub fn parse_input(
    iter: &mut dyn Iterator<Item = Result<String, std::io::Error>>,
) -> Option<Vec<usize>> {
    iter.next()
        .and_then(|s| match s {
            Ok(line) => Some(
                line.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            ),
            Err(_) => None,
        })
        .map(|s| {
            iter.take(usize::from_str(&s[0]).unwrap())
                .map(|r| {
                    r.map(|s| {
                        let input_line: Vec<usize> = s
                            .split_whitespace()
                            .map(|n| usize::from_str(n).unwrap())
                            .collect();
                        consecutive_distance(input_line)
                    })
                })
                .fold(vec![], |mut acc, i| match i {
                    Ok(result) => {
                        acc.push(result);
                        acc
                    }
                    Err(_) => acc,
                })
        })
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let maybe_results = parse_input(&mut lines);
    match maybe_results {
        Some(results) => {
            for result in results {
                println!("{:?}", result);
            }
        }
        None => println!("Empty results"),
    };
}

mod tests {
    #[test]
    fn item_distance() {
        assert_eq!(crate::item_distance(&[(0, 1), (5, 2)]), 5);
        assert_eq!(crate::item_distance(&[(0, 1), (5, 3)]), 0);
    }

    #[test]
    fn consecutive_distance() {
        let test_row_1 = vec![1, 7, 2, 11, 8, 34, 3];
        let test_row_2 = vec![31, 63, 53, 56, 96, 62, 73, 25, 54, 55, 64];
        let expected_distance_1 = 9;
        let expected_distance_2 = 26;
        assert_eq!(crate::consecutive_distance(test_row_1), expected_distance_1);
        assert_eq!(crate::consecutive_distance(test_row_2), expected_distance_2);
    }

    #[test]
    fn solution() {
        let mut test_input = (vec![
            "6 20",
            "76 74 45 48 13 75 16 14 79 58 78 82 46 89 81 88 27 64 21 63",
            "37 35 88 57 55 29 96 11 25 42 24 81 82 58 15 2 3 41 43 36",
            "54 64 52 39 36 98 32 87 95 12 40 79 41 13 53 35 48 42 33 75",
            "21 87 89 26 85 59 54 2 24 25 41 46 88 60 63 23 91 62 61 6",
            "94 66 18 57 58 54 93 53 19 16 55 22 51 8 67 20 17 56 21 59",
            "6 19 45 46 7 70 36 2 56 47 33 75 94 50 34 35 73 72 39 5",
        ])
        .into_iter()
        .map(|l| Ok(l.to_string()));
        let expected_results = vec![31, 68, 67, 52, 107, 45];
        let maybe_results = crate::parse_input(&mut test_input);
        match maybe_results {
            Some(results) => results
                .into_iter()
                .zip(expected_results.into_iter())
                .map(|(t, r)| assert_eq!(t, r))
                .for_each(drop),
            None => assert_eq!(1, 0),
        }
    }
}
