pub fn doit() {
    let res = calc(33100000);
    println!("Result of day20 p1: {}", res);
}

fn calc(input:usize) -> usize {
    let mut start = 0;
    let mut act = 1;
    loop {
        let res = blu(start + act);
        if res < input {
            if input / res > 2 {
                act *= 2;
            } else {
                act += 1;
            }
        } else if res == input {
            return act;
        } else {
            start += act / 2;
            act = 1;
        }
    }
}

fn blu(val:usize) -> usize {
    let mut res = 0;
    for j in 1..=val {
        if val % j == 0 {
            res += j*10;
        }
    }
    println!("XXX {:10} {:10}", val, res);
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_blu() {
        assert_eq!(super::blu(8), 150);
    }
    #[test]
    fn test_calc() {
        assert_eq!(super::calc(150), 8);
    }
}