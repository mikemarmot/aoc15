use std::collections::HashMap;
use std::fs;

pub fn doit() {
    let input = fs::read_to_string("data/input_day03.txt").unwrap();
    let res = calc(&input[..]);
    println!("Result of day03 p2: {}", res);
}

pub fn calc(input: &str) -> usize {
    let mut santa: bool = true;
    let mut grid = HashMap::new();
    let mut bpos = [[0,0], [0,0]];
    let mut pos = &mut bpos[0];
    grid.insert(*pos, 2);
    for c in input.chars() {
        pos = match santa {
            true => &mut bpos[0],
            false => &mut bpos[1]
        };
        match c {
            '^' => pos[1] += 1,
            'v' => pos[1] -= 1,
            '<' => pos[0] -= 1,
            '>' => pos[0] += 1,
            _ => panic!("Unknown char [{}]", c)
        };
        let cnt = match grid.get(pos) {
            Some(val) => *val,
            None => 0
        };
        grid.insert(*pos, cnt + 1);
        santa = !santa;
    }
    grid.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(super::calc(&"^v"), 3);
        assert!(super::calc(&"^>v<") == 3);
        assert!(super::calc(&"^v^v^v^v^v") == 11);
    }
}