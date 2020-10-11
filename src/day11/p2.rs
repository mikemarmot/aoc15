use std::char;

pub fn doit() {
    let input = String::from("cqjxjnds");
    println!("Result of day11 p2: {}", calc(&calc(&input)));
}

fn calc(input: &String) -> String {
    let mut input = String::from(input);
    loop {
        let mut c: Vec<u8> = input.chars().map(|x| x as u8).collect();
        for i in (0..8).rev() {
            let mut x = *c.get_mut(i).unwrap();
            x += 1;
            if x > 'z' as u8 {
                c[i] = 'a' as u8;
            } else {
                c[i] = x;
                break;
            }
        }
        let d = String::from_utf8(c).unwrap();
        if check(&d) {
            return d;
        }
        input = d;
    }
}

fn check(input: &String) -> bool {
    !input.contains('i') &&
    !input.contains('o') &&
    !input.contains('l') &&
    three_letter(input) &&
    double_letter(input)
}

fn three_letter(input: &String) -> bool {
    let mut res = false;
    for i in 0..input.len()-2 {
        let c: Vec<char> = input[i..i+3].chars().collect();
        //println!("XXX {:#?}", c);
        if c[0] as u8 + 1 == c[1] as u8 &&
           c[1] as u8 + 1 == c[2] as u8 {
               res = true;
               break;
           }
    }
    res
}

fn double_letter(input: &String) -> bool {
    let mut res = false;
    let mut first: Option<char> = None;
    for i in 0..input.len()-1 {
        let c: Vec<char> = input[i..i+2].chars().collect();
        if c[0] == c[1] && first.is_none() {
            first = Some(c[0]);
        } else if c[0] == c[1] && first.is_some() && first.unwrap() != c[0] {
            res = true;
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(super::three_letter(&String::from("abdeg")), false);
        assert_eq!(super::three_letter(&String::from("abdefhi")), true);
        assert_eq!(super::three_letter(&String::from("hijklmmn")), true);
        assert_eq!(super::three_letter(&String::from("abbceffg")), false);
        assert_eq!(super::double_letter(&String::from("xxxx")), false);
        assert_eq!(super::double_letter(&String::from("abxxdeyyzu")), true);
        assert_eq!(super::double_letter(&String::from("abbceffg")), true);
        assert_eq!(super::double_letter(&String::from("abbcegjk")), false);
        assert_eq!(super::check(&String::from("hijklmmn")), false);
        assert_eq!(super::check(&String::from("abbceffg")), false);
        assert_eq!(super::check(&String::from("abbcegjk")), false);
        assert_eq!(super::check(&String::from("abcdffaa")), true);
        assert_eq!(super::check(&String::from("ghjaabcc")), true);        
        assert_eq!(super::calc(&String::from("abcdefgh")), "abcdffaa");
        assert_eq!(super::calc(&String::from("ghijklmn")), "ghjaabcc");
    }
}