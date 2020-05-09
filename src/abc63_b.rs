use std::io::*;
use std::str::FromStr;

pub fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

use std::collections::HashMap;

fn main() {
    let s: String = read();
    let mut map: HashMap<char, bool> = HashMap::new();
    let sc: Vec<char> = s.chars().collect();
    for i in sc {
        if map.contains_key(&i) {
            println!("no");
            return
        } else {
            map.insert(i, true);
        }
    }
    println!("yes");
}
