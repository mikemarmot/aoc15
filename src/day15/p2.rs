use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day15.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input, 100);
    println!("Result of day15 p2: {}", res);
}

fn calc(input: &Vec<String>, spoons: u32) -> u64 {
    let mut ingredients:HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    for line in input {
        let base:Vec<&str> = line.split(":").collect();
        let name = base[0].trim();
        ingredients.insert(name, HashMap::new());
        let props:Vec<&str> = base[1].split(",").map(|x| x.trim()).collect();
        for p in props {
            let p:Vec<&str> = p.split(" ").collect();
            let pname = p[0];
            let pval = p[1].parse::<i32>().unwrap();
            ingredients.get_mut(name).unwrap().insert(pname, pval);
        }
    }
    let inames:Vec<&str> = ingredients.keys().map(|&x| x).collect();
    let mut props:HashMap<&str, i32> = HashMap::new();
    props.insert("capacity", 0);
    props.insert("durability", 0);
    props.insert("flavor", 0);
    props.insert("texture", 0);
    props.insert("calories", 0);
    let mut res:Vec<u64> = Vec::new();
    scalc(&ingredients, &inames, spoons, &props, &mut res);
    *res.iter().max().unwrap()
}

fn scalc(ings:&HashMap<&str,HashMap<&str,i32>>, input: &Vec<&str>, spoons: u32, props:&HashMap<&str, i32>, res:&mut Vec<u64>) {
    let mut myinput = input.clone();
    match myinput.pop() {
        Some(mying) => {
            for i in 0..spoons+1 {
                let mut myprops = props.clone();
                for (k,v) in myprops.iter_mut() {
                    *v += ings.get(mying).unwrap().get(k).unwrap() * i as i32;
                }
                scalc(ings, &myinput, spoons - i, &myprops, res);
            }
        },
        None => {
            if *props.get("calories").unwrap() == 500 {
                res.push(props.iter().filter(|(n,_)| **n != "calories").map(|(_,v)| if *v < 0 { 0 as u64 } else { *v as u64 }).product());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        let data = vec![
            String::from("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8"),
            String::from("Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3")
        ];
        assert_eq!(super::calc(&data, 100), 62842880);
    }
}