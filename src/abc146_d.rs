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

struct Edge {
    to: i32,
    id: i32,
}

fn main() {
    let n: usize = read();
    let mut e: Vec<(i64, i64)> = vec![];
    let mut cnt: Vec<i64> = vec![0; n + 1];
    let mut map: Vec<Vec<Edge>> = vec![];
    for _ in 0..n + 1 {
        map.push(vec![]);
    }
    let mut colors: Vec<i32> = vec![0; n];
    let mut done: Vec<i64> = vec![];

    for i in 0..n - 1 {
        let a: i32 = read();
        let b: i32 = read();
        map[a as usize].push(Edge {
            to: b,
            id: i as i32,
        });
        map[b as usize].push(Edge {
            to: a,
            id: i as i32,
        });
    }

    dfsNew(1, 0, 0, &map, &mut colors);
    println!("{}", colors.iter().max().unwrap());
    for i in 0..n - 1 {
        println!("{}", colors[i]);
    }
}
fn dfsNew(node: i32, pre: i32, pre_color: i32, tree: &Vec<Vec<Edge>>, colors: &mut Vec<i32>) {
    let mut c = 1;

    for i in &tree[node as usize] {
        if pre == i.to {
            continue;
        }
        if c == pre_color {
            c += 1;
        }
        colors[i.id as usize] = c;
        dfsNew(i.to, node, c, tree, colors);
        c += 1;
    }
}

// fn dfs(map: &Vec<Vec<Edge>>, edge: &Edge, done: &mut Vec<i64>, colors: &mut Vec<i64>, color: i64) {
//     let node = edge.to;
//     if done.contains(&node) {
//         return;
//     }
//     done.push(node);
//     let mut nextColor = 1;
//     if colors[edge.id as usize] == 0 {
//         colors[edge.id as usize] = color;
//     }
//     // println!("colors{:?} edge to:{}", colors, edge.to);
//     for i in &map[node as usize] {
//         if done.contains(&i.to) {
//             continue;
//         }
//         if nextColor == color {
//             nextColor += 1;
//         }
//         dfs(map, &i, done, colors, nextColor);
//         nextColor += 1;
//     }
// }

fn process(map: &mut HashMap<i32, Vec<Edge>>, a: i32, b: i32, id: i32) {
    if map.contains_key(&a) {
        let mut current = map.get_mut(&a).unwrap();
        current.push(Edge { to: b, id: id });
    } else {
        map.insert(a, vec![Edge { to: b, id: id }]);
    }
    if map.contains_key(&b) {
        let mut current = map.get_mut(&b).unwrap();
        current.push(Edge { to: a, id: id });
    } else {
        map.insert(b, vec![Edge { to: a, id: id }]);
    }
}
