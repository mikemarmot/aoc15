use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

pub fn doit() {
    let res = calc("3113322113");
    println!("Result of day10 p1: {}", res);
}

fn calc(input: &str) -> &str {
    for (i,c) in input.chars().enumerate() {
        println!("XXX {} {}", i , c);
    }
    "abc"
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(super::calc("1"), "11");
        assert_eq!(super::calc("11"), "21");
        assert_eq!(super::calc("21"), "1211");
        assert_eq!(super::calc("1211"), "111221");
        assert_eq!(super::calc("111221"), "312211");
    }
}