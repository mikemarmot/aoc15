use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day05.txt").unwrap());
    let mut res = 0;
    for line in reader.lines() {
        res += match calc(&line.unwrap()) {
            true => 1,
            false => 0
        };
    }
    println!("Result of day05 p1: {}", res);
}

pub fn calc(input: &str) -> bool {
    let mut isnice = true;
    let threev = Regex::new("[aeiou].*[aeiou].*[aeiou]").unwrap(); // three vowels
    isnice = isnice && threev.is_match(input);
    isnice = isnice && has_double(input);
    let not_allowed = Regex::new("ab|cd|pq|xy").unwrap();
    isnice = isnice && !not_allowed.is_match(input);
    isnice
}

fn has_double(input: &str) -> bool {
    let mut res = false;
    let mut last_char = '#';
    for (i,c) in input.chars().enumerate() {
        if i > 0 && c == last_char {
            res = true;
            break;
        }
        last_char = c;
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(super::calc("ugknbfddgicrmopn"), true);
        assert_eq!(super::calc("aaa"), true);
        assert_eq!(super::calc("jchzalrnumimnmhp"), false);
        assert_eq!(super::calc("haegwjzuvuyypxyu"), false);
        assert_eq!(super::calc("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_has_double() {
        assert_eq!(super::has_double("asdffffftzui"), true);
        assert_eq!(super::has_double("asdfghjkn"), false);
    }
}