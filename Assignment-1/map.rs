use std::env::args;
use std::collections::HashMap;
pub struct Map {}

impl Map {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let cnt1 = s.chars().count();
        let cnt2 = t.chars().count();
        if cnt1 != cnt2 {
            return false;
        }
        let mut str1: HashMap<char, u8> = HashMap::new();
        let mut str2: HashMap<u8, bool> = HashMap::new();
        let xs = t.as_bytes();
        for (i, c1) in s.chars().enumerate() {
            let c2 = xs[i];
            if !str1.contains_key(&c1) {
                if str2.contains_key(&c2) {
                    return false;
                }
                str1.insert(c1, c2);
                str2.insert(c2, true);
            }
            else if str1[&c1] != c2 {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let s: String = args().nth(1).unwrap();
    let t: String = args().nth(2).unwrap();
    println!("{}", Map::is_isomorphic(s,t));
}