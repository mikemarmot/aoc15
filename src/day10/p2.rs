use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day09.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day09 p2: {}", res);
}

fn calc(input: &Vec<String>) -> u32 {
    let mut cs: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for line in input {
        let sline: Vec<&str> = line.split(" ").collect();
        let (from, to, dist) = (sline[0], sline[2], sline[4].parse::<u32>().unwrap());
        if !cs.contains_key(from) {
            cs.insert(String::from(from), HashMap::new());
        }
        if !cs.contains_key(to) {
            cs.insert(String::from(to), HashMap::new());
        }
        cs.get_mut(from).unwrap().insert(String::from(to), dist);
        cs.get_mut(to).unwrap().insert(String::from(from), dist);
    }
    let mut sols: Vec<u32> = vec![];
    for start in cs.keys() {
        go(start, &cs, 0, start, &mut sols);
    }
    *sols.iter().max().unwrap()
}

fn go(start: &str, world: &HashMap<String, HashMap<String, u32>>, way: u32, fstart: &str, sols: &mut Vec<u32>) {
    let mut myworld = world.clone();
    for targets in myworld.values_mut() {
        if targets.contains_key(start) {
            targets.remove(start);
        }
    }
    let targets = myworld.remove(start).unwrap();
    if targets.len() == 0 {
        if myworld.len() == 0 { // success
            sols.push(way);
        }
    } else {
        for (target, dist) in targets.iter() {
            go(target, &myworld, way + dist, fstart, sols);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        let input: Vec<String> = [
            "London to Dublin = 464",
            "London to Belfast = 518",
            "Dublin to Belfast = 141",
                         ].iter().map(|v| String::from(*v)).collect();
        assert_eq!(super::calc(&input), 982);
    }
}