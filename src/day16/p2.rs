use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day16.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    calc(&input);
}


fn calc(input: &Vec<String>){
    let mut snap:HashMap<&str, u16> = HashMap::new();
    snap.insert("children", 3);
    snap.insert("cats", 7);
    snap.insert("samoyeds", 2);
    snap.insert("pomeranians", 3);
    snap.insert("akitas", 0);
    snap.insert("vizslas", 0);
    snap.insert("goldfish", 5);
    snap.insert("trees", 3);
    snap.insert("cars", 2);
    snap.insert("perfumes", 1);

    for sue in input {
        let mut hit = true;
        let data:Vec<&str> = sue.splitn(2, ":").collect();
        let name = data[0];
        let data:Vec<&str> = data[1].split(",").collect();
        for d in data {
            let t:Vec<&str> = d.trim().split(":").collect();
            match snap.get(t[0]) {
                Some(x) => {
                    let val = t[1].trim().parse::<u16>().unwrap();
                    if t[0] == "cats" || t[0] == "trees" {
                        if val <= *x {
                            hit = false;
                        }
                    } else if t[0] == "pomeranians" || t[0] == "goldfish" {
                        if val >= *x {
                            hit = false;
                        }
                    } else if *x != val {
                        hit = false;
                    }
                    if !hit {
                        break;
                    }
                },
                None => ()
            }
        }
        if hit {
            println!("{}", name);
        }
    }
}