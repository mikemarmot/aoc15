use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day13.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day13 p2: {}", res);
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
        let len = data[10].len() - 1;
        mm.get_mut(data[0]).unwrap().insert(&data[10][..len], val);
    }
    let keys:Vec<&str> = mm.keys().map(|&x| x).collect();
    let aaa = mm.keys().map(|&x| (x, 0 as i32)).collect::<HashMap<&str, i32>>();
    mm.insert("Myself", aaa);
    for k in keys {
        mm.get_mut(k).unwrap().insert("Myself", 0);
    }
    let mut all:Vec<Vec<String>> = Vec::new();
    let keys:Vec<String> = mm.keys().map(|&x| String::from(x)).collect();
    let path:Vec<String> = Vec::new();
    comb(&keys, &path, &mut all);
    let mut res:i32 = 0;
    for c in all {
        let mut hap:i32 = 0;
        for (i, p) in c.iter().enumerate() {
            let n = (i+c.len()-1) % c.len();
            let m = (i+1) % c.len();
            let p = mm.get(p.as_str()).unwrap();
            hap += p.get(c[n].as_str()).unwrap();
            hap += p.get(c[m].as_str()).unwrap();
        }
        if hap > res {
            res = hap;
        }
    }
    res
}

fn comb(keys:&Vec<String>, path:&Vec<String>, all:&mut Vec<Vec<String>>) {
    if keys.len() == 0 {
        all.push(path.clone());
    } else {
        for (i, key) in keys.iter().enumerate() {
            let mut mypath = path.clone();
            mypath.push(key.clone());
            let mut mykeys = keys.clone();
            mykeys.remove(i);
            comb(&mykeys, &mypath, all);
        }
    }
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