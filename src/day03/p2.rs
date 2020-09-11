use std::collections::HashMap;
use std::fs;

pub fn doit() {
    let input = fs::read_to_string("data/input_day03.txt").unwrap();
    let res = calc(&input[..]);
    println!("Result of day03 p1: {}", res);
}

pub fn calc(input: &str) -> usize {
    let mut grid = HashMap::new();
    let mut pos = (0, 0);
    grid.insert(pos, 1);
    for c in input.chars() {
        pos = match c {
            '^' => (pos.0, pos.1 + 1),
            'v' => (pos.0, pos.1 - 1),
            '<' => (pos.0 - 1, pos.1),
            '>' => (pos.0 + 1, pos.1),
            _ => panic!("Unknown char [{}]", c)
        };
        let cnt = match grid.get(&pos) {
            Some(val) => *val,
            None => 0
        };
        grid.insert(pos, cnt + 1);
    }
    grid.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert!(super::calc(&">") == 2);
        assert!(super::calc(&"^>v<") == 4);
        assert!(super::calc(&"^v^v^v^v^v") == 2);
    }
}