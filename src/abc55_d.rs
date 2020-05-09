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

fn dfs(cn: usize, s: &Vec<char>, status: &mut Vec<i64>) -> bool {
    let mut next = 0;
    if s[cn] == 'o' {
        if status[cn] == 1 {
            next = status[cn - 1];
        } else {
            next = -status[cn - 1];
        }
    }
    if s[cn] == 'x' {
        if status[cn] == 1 {
            next = -status[cn - 1];
        } else {
            next = status[cn - 1];
        }
    }
    if cn == s.len() - 1 {
        // TODO maybe checking the whole vector and return some boolean
        // return check_status(&s, &status);
        return false;
    }
    status[cn + 1] = next;

    return dfs(cn + 1, s, status);
}

fn status_print(status: &Vec<i64>) {
    let animals = status.iter().map(|&x| match x {
        -1 => 'W',
        1 => 'S',
        _ => 'E',
    });

    for i in animals {
        print!("{}", i);
    }
}

fn main() {
    let n: usize = read();
    let s: Vec<char> = read::<String>().chars().collect();

    // SS
    let mut status: Vec<i64> = vec![0; n];
    status[0] = 1;
    status[1] = 1;
    if dfs(1, &s, &mut status) {
        status_print(&status);
        return;
    }
    // SW
    let mut status: Vec<i64> = vec![0; n];
    status[0] = 1;
    status[1] = -1;
    if dfs(1, &s, &mut status) {
        status_print(&status);
        return;
    }
    // WW
    let mut status: Vec<i64> = vec![0; n];
    status[0] = -1;
    status[1] = -1;
    if dfs(1, &s, &mut status) {
        status_print(&status);
        return;
    }
    // WS
    let mut status: Vec<i64> = vec![0; n];
    status[0] = -1;
    status[1] = 1;
    if dfs(1, &s, &mut status) {
        status_print(&status);
        return;
    }
    println!("{}", -1);
}
