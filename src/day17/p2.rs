use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day17.txt").unwrap());
    let input: Vec<u8> = reader.lines().map(|l| l.unwrap().parse::<u8>().unwrap()).collect();
    let res = calc(&input,150);
    println!("Result of day17 p2: {}", res);
}

fn calc(input: &Vec<u8>, eggnog:u8) -> u16 {
    let mut res:u16 = 0;
    scalc(&input[..], eggnog, &mut res);
    res
}

fn scalc(input: &[u8], eggnog:u8, res:&mut u16) {
    for (i,c) in input.iter().enumerate() {
        if *c == eggnog {
            *res += 1;
        } else if eggnog > *c {
            scalc(&input[i+1..], eggnog - c, res)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        let data = vec![20,15,10,5,5];
        assert_eq!(super::calc(&data, 25), 4);
    }
}