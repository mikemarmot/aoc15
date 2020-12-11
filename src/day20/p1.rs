use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{HashMap,HashSet};

pub fn doit() {
    let res = calc(33100000);
    println!("Result of day20 p1: {}", res);
}

fn calc(input:usize) -> usize {
    let mut elves:usize = 0;
    let mut houses:Vec<usize> = Vec::new();
    loop {
        elves += 1;
        

    }
    12
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(super::calc(150), 8);
    }
}