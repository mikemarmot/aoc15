use std::time::SystemTime;

pub fn doit() {
    let start = SystemTime::now();
    let mut input: Vec<u8> = vec![3,1,1,3,3,2,2,1,1,3];
    for _i in 0..50 {
        input = calc(&input);
    }
    println!("Result of day10 p2: {}", input.len());
    println!("{}ms", SystemTime::now().duration_since(start).unwrap().as_millis());
}

fn calc(input: &Vec<u8>) -> Vec<u8> {
    let mut cnt = 1;
    let mut last: u8 = 0;
    let mut res = Vec::with_capacity((input.len() as f64 * 1.35).round() as usize);
    //let mut res = Vec::new();
    for i in input {
        if last == 0 {
            last = *i; 
            continue; 
        }
        if *i == last {
            cnt += 1;
        } else {
            res.push(cnt);
            res.push(last);
            last = *i;
            cnt = 1;
        }
    }
    res.push(cnt);
    res.push(last);
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(super::calc(&vec![1]), vec![1,1]);
        assert_eq!(super::calc(&vec![1,1]), vec![2,1]);
        assert_eq!(super::calc(&vec![2,1]), vec![1,2,1,1]);
        assert_eq!(super::calc(&vec![1,2,1,1]), vec![1,1,1,2,2,1]);
        assert_eq!(super::calc(&vec![1,1,1,2,2,1]), vec![3,1,2,2,1,1]);
    }
}