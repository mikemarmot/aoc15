

pub fn doit() {
    let mut input = String::from("3113322113");
    for _i in 0..40 {
        input = calc(&input);
    }
    println!("Result of day10 p1: {}", input.len());
}

fn calc(input: &String) -> String {
    let mut cnt = 1;
    let mut last = ' ';
    let mut res = String::new();
    for (i,c) in input.chars().enumerate() {
        if i == 0 {
            last = c; 
            continue; 
        }
        if c == last {
            cnt += 1;
        } else {
            res = format!("{}{}{}", res, cnt, last);
            last = c;
            cnt = 1;
        }
    }
    format!("{}{}{}", res, cnt, last)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(super::calc(&String::from("1")), "11");
        assert_eq!(super::calc(&String::from("11")), "21");
        assert_eq!(super::calc(&String::from("21")), "1211");
        assert_eq!(super::calc(&String::from("1211")), "111221");
        assert_eq!(super::calc(&String::from("111221")), "312211");
    }
}