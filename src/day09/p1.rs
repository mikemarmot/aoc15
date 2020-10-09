use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day09.txt").unwrap());
    for line in reader.lines() {
        let l = line.unwrap();
    }
    println!("Result of day09 p1: {}", calc());
}

fn calc() -> u32 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(super::calc(), 0);
    }
}