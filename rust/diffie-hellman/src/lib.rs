use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn power_mod(modulus: u64, base: u64, exp: u64) -> u64 {
    let mut base: u64= base % modulus;
    let mut exp:u64 = exp;
    let mut result: u64 = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    power_mod(p, g, a)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    power_mod(p, b_pub, a)
}
