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

fn main() {
    let n: usize = read();
    let mut cnt: Vec<i64> = vec![0; 100001];
    for _ in 0..n {
        let ai: i64 = read();
        cnt[ai as usize] += 1;
    }

    let mut cnt_two = 0;
    let mut cnt_kind = 0;

    for i in cnt {
        if i != 0 {
            cnt_kind += 1;
        }
        if i % 2 == 0 && i != 0 {
            cnt_two += 1;
        }
    }
    // println!("two:{}, kind:{}", cnt_two, cnt_kind);

    if cnt_two % 2 == 1 {
        println!("{}", cnt_kind - 1);
    } else {
        println!("{}", cnt_kind);
    }
}
