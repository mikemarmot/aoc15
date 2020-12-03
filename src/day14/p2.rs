use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

struct Reindeer {
    name:String,
    speed:u32,
    fly:u32,
    rest:u32,
}

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day14.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input, 2503);
    println!("Result of day14 p2: {}", res);
}

fn calc(input: &Vec<String>, seconds: u32) -> u32 {
    let mut rv:Vec<Reindeer> = Vec::new();
    for line in input {
        let data:Vec<&str> = line.split(" ").collect();
        let speed = data[3].parse::<u32>().unwrap();
        let fly = data[6].parse::<u32>().unwrap();
        let rest = data[13].parse::<u32>().unwrap();
        rv.push(Reindeer { name:String::from(data[0]), speed:speed, fly:fly, rest:rest });
    }
    let mut ret:HashMap<&str, u32> = HashMap::new();
    for r in &rv {
        ret.insert(&r.name, 0);
    }
    for s in 1..seconds+1 {
        let mut res:HashMap<&str, u32> = HashMap::new();
        for r in &rv {
            res.insert(&r.name, scalc(s, r.speed, r.fly, r.rest));
        }
        let max = res.values().max().unwrap();
        for (k,v) in res.iter() {
            if v == max {
                *ret.get_mut(k).unwrap() += 1;
            }
        }
    }
    *ret.values().max().unwrap()
}

fn scalc(second:u32, speed:u32, fly:u32, rest:u32) -> u32 {
    let mut mkm = second / (fly + rest) * (fly * speed);
    let mut r = second % (fly + rest);
    if r > fly {
        r = fly;
    }
    mkm += r * speed;
    mkm
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        let data = vec![
            String::from("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."),
            String::from("Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.")
        ];
        assert_eq!(super::calc(&data, 1000), 689);
    }
}