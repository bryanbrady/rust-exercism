pub fn is_armstrong_number(num: u32) -> bool {
    let mut acc = 0_u32;
    let mut n = num;
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= 10;
    }
    n = num;
    while n > 0 {
        let q = n / 10;
        let r = n % 10;
        n = q;
        acc += r.pow(digits);
    }
    if acc == num { true } else { false }
}
