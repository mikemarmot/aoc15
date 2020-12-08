use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{HashMap,HashSet};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day19.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let (d1, d2) = prep_data(&input);
    let res = calc(&d1, d2);
    println!("Result of day19 p2: {}", res);
}

fn prep_data(input: &Vec<String>) -> (HashMap<&str, Vec<&str>>, &str) {
    let mut rules:HashMap<&str, Vec<&str>> = HashMap::new();
    let mut start:Option<&str> = None;
    for line in input {
        if line.contains(" => ") {
            let rp:Vec<&str> = line.split(" => ").collect();
            match rules.get_mut(rp[0]) {
                Some(x) => x.push(rp[1]),
                None => { rules.insert(rp[0], vec![rp[1]]); }
            };

        } else if line.len() > 0 {
            start = Some(&line[..]);
        }
    }
    (rules, start.unwrap())
}

fn calc(input: &HashMap<&str, Vec<&str>>, d:&str) -> usize {
    let mut res:HashSet<String> = HashSet::new();
    for i in 0..d.len() {
        for (ik, iv) in input.iter().filter(|(&k,_)| i+k.len() <= d.len() && &d[i..i+k.len()] == k) {
            for v in iv {
                res.insert(String::from(format!("{}{}{}", &d[..i], v, &d[i+ik.len()..])));
            }
        }
    }
    res.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        let data = vec![
            String::from("H => HO"),
            String::from("H => OH"),
            String::from("O => HH"),
            String::from(""),
            String::from("HOH")
        ];
        let (d1, d2) = super::prep_data(&data);
        assert_eq!(super::calc(&d1, d2), 4);
    }
}