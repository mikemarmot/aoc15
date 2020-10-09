use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day08.txt").unwrap());
    let mut res_code = 0;
    let mut res_mem = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        res_code += calc_code(&l);
        res_mem += calc_mem(&l);
    }
    println!("Result of day08 p2: {}", res_mem - res_code);
}

pub fn calc_code(input: &str) -> usize {
    input.len()
}

pub fn calc_mem(input: &str) -> usize {
    let mut res = 0;
    for c in input.chars() {
        if c == '"' {
            res += 2;
        } else if c == '\\' {
            res += 2
        }else {
            res += 1;
        }
    }
    res + 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc_code() {
        assert_eq!(super::calc_code("\"\""), 2);
        assert_eq!(super::calc_code("\"abc\""), 5);
        assert_eq!(super::calc_code("\"aaa\\\"aaa\""), 10);
        assert_eq!(super::calc_code("\"\\x27\""), 6);
    }

    #[test]
    fn test_calc_mem() {
        assert_eq!(super::calc_mem("\"\""), 6);
        assert_eq!(super::calc_mem("\"abc\""), 9);
        assert_eq!(super::calc_mem("\"aaa\\\"aaa\""), 16);
        assert_eq!(super::calc_mem("\"\\x27\""), 11);
    }
}