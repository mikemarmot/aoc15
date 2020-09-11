use std::fs;

pub fn doit() {
    let input = fs::read_to_string("data/input_day01.txt").unwrap();
    let res = calc(&input[..]);
    println!("Result of day01 p1: {}", res);
}

pub fn calc(input: &str) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!()
        };
    }
    floor
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert!(super::calc(&"") == 0);
        assert!(super::calc(&"(())") == 0);
        assert!(super::calc(&"()()") == 0);
        assert!(super::calc(&"(((") == 3);
        assert!(super::calc(&"(()(()(") == 3);
        assert!(super::calc(&"))(((((") == 3);
        assert!(super::calc(&"())") == -1);
        assert!(super::calc(&"))(") == -1);
        assert!(super::calc(&")))") == -3);
        assert!(super::calc(&")())())") == -3);
    }
}