use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day02.txt").unwrap());
    let mut res = 0;
    for line in reader.lines() {
        res += calc(&line.unwrap());
    }
    println!("Result of day02 p2: {}", res);
}

pub fn calc(input: &str) -> i32 {
    let mut vec: Vec<i32> = input.split("x").map(|x| x.parse::<i32>().unwrap()).collect();
    vec.sort();
    2 * vec[0] + 2 * vec[1] + vec[0] * vec[1] * vec[2]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert!(super::calc(&"2x3x4") == 34);
        assert!(super::calc(&"1x1x10") == 14);
    }
}