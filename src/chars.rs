use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static UNICODE: Lazy<HashMap<u32, Vec<&str>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    static CHARS: &str = include_str!("../data/UnicodeData.txt");
    for line in CHARS.trim().split("\n") {
        let row = line.split(";").collect::<Vec<&str>>();
        let code = i64::from_str_radix(row[0], 16).expect(&line.to_string()) as u32;
        m.insert(code, row[1..].iter().map(|s|*s).collect());
    }
    m
});
