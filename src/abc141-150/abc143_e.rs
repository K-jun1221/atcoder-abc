use std::io::*;
use std::str::FromStr;s

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
    let n = read::<i64>();
    let m = read::<i64>();
    let l = read::<i64>();
    let mut map: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();

    for _ in 0..m {
        let a = read::<i64>();
        let b = read::<i64>();
        let w = read::<i64>();
        if map.contains_key(&a) {
            let current = map.get_mut(&a).unwrap();
            current.push((b, w))
        } else {
            map.insert(a, vec![(b, w)]);
        }

        if map.contains_key(&b) {
            let current = map.get_mut(&b).unwrap();
            current.push((a, w))
        } else {
            map.insert(b, vec![(a, w)]);
        }
    }


    let q = read::<i64>();
    let mut qs: Vec<(i64, i64)> = vec![];

    for _ in 0..q {
        let from = read::<i64>();
        let to = read::<i64>();
        qs.push((from, to))
    }

    // println!("{:?}", map);
    // println!("{:?}", qs);

    for i in qs {
        let mut minpath: i64 = 0;
        let mut visited = vec![];
        let mut oil = l;
        let maxOil: i64 = l;
        let targetNode: i64 = i.1;
        let ans = dfs(
            i.0,
            &mut visited,
            &map,
            &mut minpath,
            &mut oil,
            maxOil,
            targetNode,
        );
        if ans == 10000000000 {
            println!("{}", -1)
        } else {
            println!("{}", ans)
        }

    }
}

fn dfs(
    node: i64,
    visited: &mut Vec<i64>,
    map: &HashMap<i64, Vec<(i64, i64)>>,
    minpath: &mut i64,
    oil: &mut i64,
    maxOil: i64,
    targetNode: i64,
) -> i64 {
    if visited.contains(&node) {
        return 10000000;
    }
    if node == targetNode {
        return *minpath;
    }

    // println!("node: {}", node);

    let mut min_value = 100000000;
    visited.push(node);
    if map.contains_key(&node) {
        for (i, v) in map.get(&node).unwrap().iter().enumerate() {
            if maxOil < v.1 {
                continue;
            }

            let mut new_minpath = minpath.clone();
            let mut new_oil = oil.clone();
            if *oil < v.1 {
                // 色々処理
                new_oil = maxOil;
                new_minpath += 1;
            }

            new_oil -= v.1;

            // println!("oil: {}", oil);

            let kouho = dfs(
                v.0 as i64,
                visited,
                map,
                &mut new_minpath,
                &mut new_oil,
                maxOil,
                targetNode,
            );
            min_value = min(min_value, kouho)
        }
    }
    return min_value;
}