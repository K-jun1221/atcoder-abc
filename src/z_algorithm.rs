pub fn zalgorithm(s: &str) -> Vec<usize> {
    let s_char: Vec<char> = s.chars().collect();
    let mut a = vec![0; s_char.len()];

    a[0] = s_char.len();
    let mut i = 1;
    let mut j = 0;

    while i < s_char.len() {
        while i + j < s_char.len() && s_char[j] == s_char[i + j] {
            j += 1;
        }
        a[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while i + j < s_char.len() && k + a[k] < j {
            k += 1;
        }
        i += k;
        j -= k;
    }

    return a;
}
