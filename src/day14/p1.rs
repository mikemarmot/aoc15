use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day14.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input, 2503);
    println!("Result of day14 p1: {}", res);
}

fn calc(input: &Vec<String>, seconds: u32) -> u32 {
    let mut km:u32 = 0;
    for line in input {
        let data:Vec<&str> = line.split(" ").collect();
        let speed = data[3].parse::<u32>().unwrap();
        let fly = data[6].parse::<u32>().unwrap();
        let rest = data[13].parse::<u32>().unwrap();
        let mut mkm = seconds / (fly + rest) * (fly * speed);
        let mut r = seconds % (fly + rest);
        if r > fly {
            r = fly;
        }
        mkm += r * speed;
        if mkm > km {
            km = mkm;
        }
    }
    km
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        let data = vec![
            String::from("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."),
            String::from("Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.")
        ];
        assert_eq!(super::calc(&data, 1000), 1120);
    }
}