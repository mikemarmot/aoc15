use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

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
    let mut size:usize = d.len();
    calc(input, "e", d, 1, &mut res, &mut size);
    *res.iter().min().unwrap()
}

fn calc(input: &HashMap<&str, &str>, s:&str, d:&str, c:usize, res:&mut Vec<usize>, size:&mut usize) {
    for i in 0..d.len() {
        for (ik, iv) in input.iter().filter(|(&k,_)| i+k.len() <= d.len() && &d[i..i+k.len()] == k) {
            println!("c:{} i:{} k:{} -> {}", c, i, ik, iv);
            let t = format!("{}{}{}", &d[..i], iv, &d[i+ik.len()..]);
            if t.len() < *size { *size = t.len(); print!("{:5}\r", size)}
            if t == s {
                res.push(c);
                //print!("{} ",c);
            } else {
                calc(input, s, &t, c+1, res, size);
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