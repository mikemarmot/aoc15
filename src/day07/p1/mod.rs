use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;

pub mod wirebox;
use wirebox::{WireBox, Wire};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day07.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day07 p1: {:#?}", res.get("a").unwrap());
}

pub fn wire<'a>(catalog: &HashMap<String, HashMap<String, String>>, wb: &'a mut WireBox, name: &str) -> &'a Wire {
    if wb.get_wire(name).is_some() {
        return wb.get_wire(name).unwrap();
    }

    let coll = catalog.get(name).unwrap();

    let val: u16;
    if let Some(x) = coll.get("signal") {
        val = x.parse().unwrap();
    } else if let Some(x) = coll.get("iwire") {
        let w = match wb.get_wire(name) {
            Some(w) => w,
            None => wire(catalog, wb, x)
        };
        val = w.value;
    } else if coll.contains_key("gate") {
        let op = coll.get("op").unwrap();

        let rval: u16;
        if let Some(x) = coll.get("rval") {
            rval = x.parse().unwrap();
        } else if let Some(x) = coll.get("rwire") {
            let w = match wb.get_wire(name) {
                Some(w) => w,
                None => wire(catalog, wb, x)
            };
            rval = w.value;
        } else {
            panic!("invalid expression");
        }

        let mut lval: Option<u16> = None;
        if let Some(x) = coll.get("lval") {
            lval = Some(x.parse().unwrap());
        } else if let Some(x) = coll.get("lwire") {
            let w = match wb.get_wire(name) {
                Some(w) => w,
                None => wire(catalog, wb, x)
            };
            lval = Some(w.value);
        }
        val = calc_signal(lval, op, rval);
    } else {
        panic!("invalid expression");
    }
    wb.create_wire(name, val)
}

fn calc_signal(left: Option<u16>, op: &str, right: u16) -> u16 {
    match op {
        "AND" => left.unwrap() & right,
        "OR" => left.unwrap() | right,
        "NOT" => !right,
        "LSHIFT" => left.unwrap() << right,
        "RSHIFT" => left.unwrap() >> right,
        _ => panic!("unknown operator [{}]", op),
    }
}

fn resolve(left: &str) -> HashMap<String, String> {
    lazy_static! {
        static ref RE_L: Regex = Regex::new(r"^(?P<signal>\d+)$|^(?P<iwire>[a-z]+)$|^(?P<gate>(?P<left>(?:(?P<lval>\d+)|(?P<lwire>[a-z]+)) )?(?P<op>[A-Z]+) (?P<right>(?P<rval>\d+)|(?P<rwire>[a-z]+)))$").unwrap();
    }
    let mut coll: HashMap<String, String> = HashMap::new();
    let caps = RE_L.captures(left).unwrap();
    RE_L.capture_names()
        .filter_map(|n| n)
        .filter(|n| caps.name(n).is_some())
        .for_each(|n| { coll.insert(String::from(n), String::from(caps.name(n).unwrap().as_str())); } );
    coll
}

pub fn calc(exprs: &Vec<String>) -> HashMap<String, u16> {
    let mut wb = WireBox::new();

    let re_full: Regex = Regex::new(r"^(?P<left>.+) -> (?P<right>[a-z]+)$").unwrap();
    let mut catalog: HashMap<String, HashMap<String, String>> = HashMap::new();
    for input in exprs {
        let caps = re_full.captures(input).unwrap();
        catalog.insert(String::from(caps.name("right").unwrap().as_str()), resolve(caps.name("left").unwrap().as_str()));
    }

    catalog.keys().for_each(|k| { wire(&catalog, &mut wb, k); });
    wb.get_all_wires()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calc() {
        let input: Vec<String> = ["123 -> x",
                         "456 -> y",
                         "x AND y -> d",
                         "x OR y -> e",
                         "x LSHIFT 2 -> f",
                         "y RSHIFT 2 -> g",
                         "NOT x -> h",
                         "NOT y -> i",
                         ].iter().map(|v| String::from(*v)).collect();
        let res = super::calc(&input);
        assert_eq!(res.len(), 8);
        assert_eq!(*res.get("d").unwrap(), 72);
    }
}