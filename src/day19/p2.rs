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
    let mut res:Vec<usize> = Vec::new();
    let mut cnt_f:u64 = 0;
    let mut cnt_t:u64 = 0;
    calc(input, d, "e", 1, &mut res, &mut cnt_f, &mut cnt_t);
    *res.iter().min().unwrap()
}

fn calc(input: &HashMap<&str, Vec<&str>>, s:&str, d:&str, c:usize, res:&mut Vec<usize>, cnt_f:&mut u64, cnt_t:&mut u64) {
    for i in 0..d.len() {
        for (ik, iv) in input.iter().filter(|(&k,_)| i+k.len() <= d.len() && &d[i..i+k.len()] == k) {
            for v in iv {
                if (1..=475).contains(&c) { println!("{}:{}", c, ik)}
                let t = format!("{}{}{}", &d[..i], v, &d[i+ik.len()..]);
                if t == s {
                    res.push(c);
                    *cnt_t += 1;
                } else if t.len() >= s.len() {
                    *cnt_f += 1;
                    //if *cnt_f % 1000 == 0 { print!("{:10}//{:10}\r", cnt_f, cnt_t); }
                } else {
                    calc(input, s, &t, c+1, res, cnt_f, cnt_t);
                }
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