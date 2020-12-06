use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day17.txt").unwrap());
    let input: Vec<u8> = reader.lines().map(|l| l.unwrap().parse::<u8>().unwrap()).collect();
    let res = calc(&input,150);
    println!("Result of day17 p2: {}", res);
}

fn calc(input: &Vec<u8>, eggnog:u8) -> usize {
    let mut res:Vec<Vec<u8>> = Vec::new();
    scalc(&input[..], eggnog, &Vec::new(), &mut res);
    let no = res.iter().map(|x| x.len()).min().unwrap();
    no
}

fn scalc(input: &[u8], eggnog:u8, path:&Vec<u8>, res:&mut Vec<Vec<u8>>) {
    for (i,c) in input.iter().enumerate() {
        if *c == eggnog {
            res.push(path.to_vec());
        } else if eggnog > *c {
            let mut mypath = path.clone();
            mypath.push(*c);
            scalc(&input[i+1..], eggnog - c, &mypath, res);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        let data = vec![20,15,10,5,5];
        assert_eq!(super::calc(&data, 25), 3);
    }
}