use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::cmp::Reverse;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day19.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let (d1, d2) = prep_data(&input);
    let res = scalc(&d1, d2);
    println!("Result of day19 p2: {}", res);
}

fn prep_data(input: &Vec<String>) -> (HashMap<&str, &str>, &str) {
    let mut rules:HashMap<&str, &str> = HashMap::new();
    let mut start:Option<&str> = None;
    for line in input {
        if line.contains(" => ") {
            let rp:Vec<&str> = line.split(" => ").collect();
            match rules.get_mut(rp[1]) {
                Some(x) => println!("whahh {}", x),
                None => { rules.insert(rp[1], rp[0]); }
            };
        } else if line.len() > 0 {
            start = Some(&line[..]);
        }
    }
    (rules, start.unwrap())
}

fn scalc(input: &HashMap<&str, &str>, d:&str) -> usize {
    let mut res:Vec<usize> = Vec::new();
    let mut inkeys:Vec<&str> = input.keys().map(|&x| x).collect();
    inkeys.sort_by_key(|x| Reverse(x.len()));
    //println!("{:?}", inkeys);
    calc(&inkeys, input, d, 0, &mut res);
    //println!("{:?}",res);
    *res.iter().min().unwrap()
}

fn calc(inkeys:&Vec<&str>, input: &HashMap<&str, &str>, d:&str, size:usize, res:&mut Vec<usize>) {
    //println!("{}: {} {:?}", size, d, res);
    //print!("{:10}\r", size);
    if d.contains("e") {
        if d.len() == 1 {
            res.push(size);
            //println!("success {}", size);
        }
    } else if res.len() >= 100000 {
        // nothing
    } else {
        for k in inkeys {
            if d.find(k).is_some() {
                let md = d.replacen(k, input.get(k).unwrap(), 1);
                calc(inkeys, input, &md, size+1, res);
            }
        }
    }
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