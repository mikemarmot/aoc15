use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;

enum Cmd {
    ON,
    OFF,
    TOGGLE,
}

impl Cmd {
    fn from_str(s: &str) -> Option<Cmd> {
        match s {
            "turn on" => Some(Cmd::ON),
            "turn off" => Some(Cmd::OFF),
            "toggle" => Some(Cmd::TOGGLE),
            _ => None
        }
    }
}

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day06.txt").unwrap());
    let mut grid: [[bool;1000];1000] = [[false;1000];1000];
    for line in reader.lines() {
        calc(&line.unwrap(), &mut grid);
    }
    let res = grid.iter().map(|x| x.iter().map(|y| match y { true => 1, false => 0 }).sum::<u32>()).sum::<u32>();
    println!("Result of day06 p1: {}", res);
}

pub fn calc(input: &str, grid: &mut [[bool;1000];1000]) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.+) (\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})").unwrap();
    }
    let caps = RE.captures(input).unwrap();
    assert_eq!(caps.len(), 6);
    let (cmd, x1, y1, x2, y2) = (Cmd::from_str(&caps[1]).unwrap(),
                                    *&caps[2].parse::<usize>().unwrap(),
                                    *&caps[3].parse::<usize>().unwrap(),
                                    *&caps[4].parse::<usize>().unwrap(),
                                    *&caps[5].parse::<usize>().unwrap());
    let c = match cmd {
        Cmd::ON => |_y:bool| true,
        Cmd::OFF => |_y:bool| false,
        Cmd::TOGGLE => |y:bool| !y,
    };
    for x in x1..x2+1 {
        for y in y1..y2+1 {
            grid[x][y] = c(grid[x][y]);
        }
    }
}


#[cfg(test)]
mod tests {
}