use md5;

pub fn doit() {
    let input = "bgvyzdsv";
    let res = calc(&input);
    println!("Result of day04 p1: {}", res);
}

pub fn calc(input: &str) -> u32 {
    let mut res = 0;
    loop {
        res += 1;
        let digest = format!("{:x}", md5::compute((String::from(input) + &res.to_string()).as_bytes()));
        if digest.starts_with("00000") {
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(format!("{:x}", md5::compute(b"abcdefghijklmnopqrstuvwxyz")), "c3fcd3d76192e4007dfb496cca67e13b");
        assert_eq!(super::calc("abcdef"), 609043);
        assert_eq!(super::calc("pqrstuv"), 1048970);
    }
}