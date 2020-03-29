const MODULO: i64 = 1_000_000_000 + 7;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse::<i64>().unwrap());
    let n = iter.next().unwrap();
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();

    println!(
        "{}",
        ((powmod(2, n) - 1 - combination(n, a) + MODULO) % MODULO - combination(n, b) + MODULO) % MODULO
    );
}
fn fact(x: i64) -> i64 {
    let mut ret: i64 = 1;
    for i in 1..x + 1 {
        ret = ret * i % MODULO;
    }
    return ret;
}
fn powmod(x: i64, n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    if n % 2 == 0 {
        let k = powmod(x, n / 2);
        return (k * k) % MODULO;
    } else {
        let k = powmod(x, n / 2);
        return (((k * k) % MODULO) * x) % MODULO;
    }
}
#[test]
fn test_combination() {
    assert_eq!(combination(6, 3), (6 * 5 * 4) / (3 * 2 * 1));
    assert_eq!(fact(5), 5 * 4 * 3 * 2 * 1);
    assert_eq!(combination(4, 1), 4);
    assert_eq!(combination(4, 3), 4);
    assert_eq!(powmod(2, 4), 16);
}

// aCb の計算
fn combination(a: i64, b: i64) -> i64 {
    let mut ret = 1;
    for i in 0..b {
        ret = ret * (a - i) % MODULO;
    }
    return ret * inverse(fact(b)) % MODULO;
}

// x ^ -1　(in modulo) の計算
fn inverse(x: i64) -> i64 {
    powmod(x, MODULO - 2)
}
