use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day02.txt").unwrap());
    let mut res = 0;
    for line in reader.lines() {
        res += calc(&line.unwrap());
    }
    println!("Result of day02 p1: {}", res);
}

pub fn calc(input: &str) -> i32 {
    let vec: Vec<i32> = input.split("x").map(|x| x.parse::<i32>().unwrap()).collect();
    let (l, w, h) = (vec[0], vec[1], vec[2]);
    let f = [l * w, w * h, h * l];
    f.iter().map(|x| x * 2).sum::<i32>() + f.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert!(super::calc(&"2x3x4") == 58);
        assert!(super::calc(&"1x1x10") == 43);
    }
}