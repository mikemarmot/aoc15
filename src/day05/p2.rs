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
    println!("Result of day05 p2: {}", res);
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
    for i in 1..input.len() {
        let s = &input[i-1..i+1];
        if input[i+1..].contains(s) {
            res = true;
            break;
        }
    }
    res
}

fn one_between(input: &str) -> bool {
    let mut res = false;
    for i in 1..input.len()-2 {
    }
    res

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(super::calc("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(super::calc("xxyxx"), true);
        assert_eq!(super::calc("uurcxstgmygtbstg"), false);
        assert_eq!(super::calc("ieodomkazucvgmuy"), false);
    }

    #[test]
    fn test_has_double() {
        assert_eq!(super::has_double("asfdxztzuixzhjfkd"), true);
        assert_eq!(super::has_double("aaa"), false);
    }
}