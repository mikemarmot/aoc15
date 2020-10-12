use serde_json::Value;
use std::fs;

pub fn doit() {
    let input = fs::read_to_string("data/input_day12.txt").unwrap();
    println!("Result of day12 p1: {}", calc(&input));
}

fn calc(input: &String) -> i64 {
    let v: Value = serde_json::from_str(input).unwrap();
    evaluate(&v)
}

fn evaluate(v: &Value) -> i64 {
    let mut res: i64 = 0;
    match v {
        Value::Object(m) => for x in m.values() { res += evaluate(x); },
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
        assert_eq!(super::calc(&String::from(r#"{"a":2,"b":4}"#)), 6);
        assert_eq!(super::calc(&String::from(r#"[[[3]]]"#)), 3);
        assert_eq!(super::calc(&String::from(r#"{"a":{"b":4},"c":-1}"#)), 3);
        assert_eq!(super::calc(&String::from(r#"{"a":[-1,1]}"#)), 0);
        assert_eq!(super::calc(&String::from(r#"[-1,{"a":1}]"#)), 0);
        assert_eq!(super::calc(&String::from(r#"[]"#)), 0);
        assert_eq!(super::calc(&String::from(r#"{}"#)), 0);
    }
}