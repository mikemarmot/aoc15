use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{HashMap,HashSet};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day19.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let (d1, d2) = prep_data(&input);
    let res = scalc(&d1, d2);
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

fn scalc(input: &HashMap<&str, Vec<&str>>, d:&str) -> usize {
    // println!("XXX1 {:#?}", d);
    let mut res:Vec<String> = Vec::new();
    res.push(String::from("e"));
    let mut step:usize = 0;
    while !res.contains(&String::from(d)) {
        println!("XXX2 {:#?}", res.len());
        let mut r:Vec<String> = Vec::new();
        for e in res {
            let re = calc(&input, &e);
            r.extend(re);
        }
        r.sort();
        r.dedup();
        res = r;
        step += 1;
    }
    step
}

fn calc(input: &HashMap<&str, Vec<&str>>, d:&str) -> Vec<String> {
    let mut res:Vec<String> = Vec::new();
    for i in 0..d.len() {
        for (ik, iv) in input.iter().filter(|(&k,_)| i+k.len() <= d.len() && &d[i..i+k.len()] == k) {
            for v in iv {
                res.push(String::from(format!("{}{}{}", &d[..i], v, &d[i+ik.len()..])));
            }
        }
    }
    // println!("XXX3 {:#?}", res);
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc1() {
        let data = vec![
            String::from("e => H"),
            String::from("e => O"),
            String::from("H => HO"),
            String::from("H => OH"),
            String::from("O => HH"),
            String::from(""),
            String::from("HOH")
        ];
        let (d1, d2) = super::prep_data(&data);
        assert_eq!(super::scalc(&d1, d2), 3);
    }

    #[test]
    fn test_calc2() {
        let data = vec![
            String::from("e => H"),
            String::from("e => O"),
            String::from("H => HO"),
            String::from("H => OH"),
            String::from("O => HH"),
            String::from(""),
            String::from("HOHOHO")
        ];
        let (d1, d2) = super::prep_data(&data);
        assert_eq!(super::scalc(&d1, d2), 6);
    }    
}