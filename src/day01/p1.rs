use std::fs;

pub fn doit() {
    let input = fs::read_to_string("data/input_day01.txt").unwrap();
    let res = calc(&input[..]);
    println!("Result of day01 p2: {}", res);
}

pub fn calc(input: &str) -> usize {
    let mut floor = 0;
    let mut res = 0;
    for (i, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!()
        };
        if floor == -1 {
            res = i + 1;
            break;
        }
    }
    res
}

mod tests {
    #[test]
    fn test_calc() {
        assert!(super::calc(&"") == 0);
        assert!(super::calc(&"())") == 3);
        assert!(super::calc(&"))(") == 1);
    }
}