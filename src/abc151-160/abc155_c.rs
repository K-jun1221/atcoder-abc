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

pub fn read_n<T: FromStr>(n: usize) -> Vec<T> {
    read_n_logic::<T>(n, vec![])
}

pub fn read_n_logic<T: FromStr>(n: usize, mut a: Vec<T>) -> Vec<T> {
    match n {
        0 => a,
        _ => {
            a.push(read::<T>());
            read_n_logic(n - 1, a)
        }
    }
}
use std::cmp::{max, min};
use std::collections::HashMap;

fn main() {
    let n: usize = read();
    let s: Vec<String> = read_n(n);

    let mut map: HashMap<String, i64> = HashMap::new();
    for i in 0..s.len() {
        let si: &str = &s[i];
        if map.contains_key(&(si.to_string())) {
            let current = map.get_mut(&(si.to_string())).unwrap();
            *current += 1;
        } else {
            map.insert(si.to_string(), 1);
        }
    }


    // what is max
    let mut maxv = 0;
    for (k,v) in map.iter() {
        if maxv < *v {
            maxv = *v;
        }
    }

    // get max values strings
    let mut anss: Vec<&str> = vec![];
    for (k, v) in map.iter() {
        if *v == maxv {
            anss.push(k);
        }
    }

    anss.sort();
    for i in 0..anss.len() {
        println!("{}", anss[i]);
    }

}
