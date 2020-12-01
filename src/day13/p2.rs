use serde_json::Value;
use std::fs;

pub fn doit() {
    let input = fs::read_to_string("data/input_day12.txt").unwrap();
    println!("Result of day13 p2: {}", calc(&input));
}

fn calc(input: &String) -> i64 {
    let v: Value = serde_json::from_str(input).unwrap();
    evaluate(&v)
}

fn evaluate(v: &Value) -> i64 {
    let mut res: i64 = 0;
    match v {
        Value::Object(m) =>  {
            let a: Vec<&str> = m.values().map(|p| p.as_str().unwrap_or("")).collect();
            if !a.contains(&"red") {
                for x in m.values() {
                    res += evaluate(x);
                }
            }
        },
        Value::Array(a) => for x in a { res += evaluate(x); },
        Value::Number(x) => res += x.as_i64().unwrap_or(0),
        _ => (),
    };
    res
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(super::calc(&String::from(r#"[1,2,3]"#)), 6);
        assert_eq!(super::calc(&String::from(r#"[1,{"c":"red","b":2},3]"#)), 4);
        assert_eq!(super::calc(&String::from(r#"{"d":"red","e":[1,2,3,4],"f":5}"#)), 0);
        assert_eq!(super::calc(&String::from(r#"[1,"red",5]"#)), 6);
    }
}