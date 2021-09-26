fn collatz_rec(n: u64, i: u64) -> u64 {
    match (n, n % 2 == 0) {
        (1, _) => i,
        (_, true) => collatz_rec(n >> 1, i+1),
        (_, false) => collatz_rec(3 * n + 1, i+1)
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        _ => Some(collatz_rec(n, 0))
    }
}
