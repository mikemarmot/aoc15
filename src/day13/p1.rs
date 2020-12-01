use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day07.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day13 p1: {}", res);
}

fn calc(input: &Vec<String>) -> i32 {
    let mut mm: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    for line in input {
        let data :Vec<&str> = line.split(" ").collect();
        if !mm.contains_key(data[0]) {
            mm.insert(data[0], HashMap::new());
        }
        let mut val = data[3].parse::<i32>().unwrap();
        if data[2] == "lose" {
            val = val * -1;
        }
        mm.get_mut(data[0]).unwrap().insert(data[10], val);
        println!("all components {} {} {} {} {}", data[0], data[2], data[3], data[10], val)
    }
    12
}




#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        let data = vec![
            String::from("Alice would gain 54 happiness units by sitting next to Bob."),
            String::from("Alice would lose 79 happiness units by sitting next to Carol."),
            String::from("Alice would lose 2 happiness units by sitting next to David."),
            String::from("Bob would gain 83 happiness units by sitting next to Alice."),
            String::from("Bob would lose 7 happiness units by sitting next to Carol."),
            String::from("Bob would lose 63 happiness units by sitting next to David."),
            String::from("Carol would lose 62 happiness units by sitting next to Alice."),
            String::from("Carol would gain 60 happiness units by sitting next to Bob."),
            String::from("Carol would gain 55 happiness units by sitting next to David."),
            String::from("David would gain 46 happiness units by sitting next to Alice."),
            String::from("David would lose 7 happiness units by sitting next to Bob."),
            String::from("David would gain 41 happiness units by sitting next to Carol.")
        ];
        assert_eq!(super::calc(&data), 330);
    }
}