use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day18.txt").unwrap());
    let input: Vec<Vec<bool>> = reader.lines().map(|l| l.unwrap().chars().map(|c| c == '#').collect()).collect();
    let res = calc(&input, 100);
    println!("Result of day18 p1: {}", res);
}

fn calc(input: &Vec<Vec<bool>>, steps:usize) -> usize {
    let mut old = input.clone();
    let mut new = input.clone();
    //println!("{:?}", old);
    for _i in 1..=steps {
        for (y, v) in input.iter().enumerate() {
            for (x, _e) in v.iter().enumerate() {
                new[y][x] = val(&old, x, y);
            }
        }
        // println!("{:?}", new);
        old = new.clone();
    }
    new.iter().map(|x| x.iter().map(|x| if *x { 1 } else { 0 }).sum::<usize>()).sum::<usize>()
}

fn val(input: &Vec<Vec<bool>>, xpos:usize, ypos:usize) -> bool {
    let mut a:[bool; 8] = [false; 8];
    let state = input[ypos][xpos];
    let xpos = xpos as i16;
    let ypos = ypos as i16;
    // println!("SSS {} {} {}", xpos, ypos, input[ypos][xpos]);
    a[0] = f(input, xpos -1, ypos -1); // lo
    a[1] = f(input, xpos, ypos -1); // o
    a[2] = f(input, xpos +1, ypos -1); // ro
    a[3] = f(input, xpos +1, ypos); // r
    a[4] = f(input, xpos +1, ypos +1); // ru
    a[5] = f(input, xpos, ypos +1); // u
    a[6] = f(input, xpos -1, ypos +1); // lu
    a[7] = f(input, xpos -1, ypos); // l

    let c:u8 = a.iter().map(|b| if *b { 1 } else { 0 }).sum();
    // if xpos == 2 && ypos == 5 {
    //     println!("YYY {:?} {}", a, c);
    // }
    match state {
        true => (2..=3).contains(&c),
        false => c == 3,
    }
}

fn f(input: &Vec<Vec<bool>>, xpos:i16, ypos:i16) -> bool {
    let (minx, maxx) = (0 as i16, input[0].len() as i16 - 1);
    let (miny, maxy) = (0 as i16, input.len() as i16 - 1);
    if xpos > maxx || ypos > maxy ||
        xpos < minx || ypos < miny {
            false
    } else {
        input[ypos as usize][xpos as usize]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        let data = vec![
            vec![false, true, false, true, false, true],
            vec![false, false, false, true, true, false],
            vec![true, false, false, false, false, true],
            vec![false, false, true, false, false, false],
            vec![true, false, true, false, false, true],
            vec![true, true, true, true, false, false],
        ];
        assert_eq!(super::calc(&data, 4), 4);
    }
}