pub fn nth(n: u32) -> u32 {
    let mut i = n;
    let mut primes: Vec<u32> = Vec::new();
    let mut p = 6;

    match n {
        0 => return 2,
        1 => return 3,
        2 => return 5,
        _ => {
            loop {
                p += 1;
                if !(p % 2 == 0 || p % 3 == 0 || p % 5 == 0) {
                    if !primes.iter().any(|x| p % x == 0) {
                        primes.push(p);
                        i -= 1;
                        if i <= 2 {
                            break;
                        }
                    }
                }
            }
            return p;
        }
    }
}
