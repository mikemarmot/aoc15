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
    println!("Result of day08 p1: {}", res_code - res_mem);
}

pub fn calc_code(input: &str) -> usize {
    input.len()
}

pub fn calc_mem(input: &str) -> usize {
    let mut res = 0;
    let mut pos = 0;
    loop {
        if input.len() > pos + 2 && &input[pos..pos+2] == "\\x" {
            pos += 4;
            res += 1;
        } else if input.len() > pos + 1 && &input[pos..pos+2] == "\\\"" {
            pos += 2;
            res += 1;
        } else if input.len() > pos + 1 && &input[pos..pos+2] == "\\\\" {
            pos += 2;
            res += 1;
        } else if input.len() > pos && &input[pos..pos+1] == "\"" {
            pos += 1;
        } else {
            pos += 1;
            res += 1;
        }

        if pos >= input.len() {
            break;
        }
    }
    res
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
        assert_eq!(super::calc_mem("\"\""), 0);
        assert_eq!(super::calc_mem("\"abc\""), 3);
        assert_eq!(super::calc_mem("\"aaa\\\"aaa\""), 7);
        assert_eq!(super::calc_mem("\"\\x27\""), 1);
    }
}