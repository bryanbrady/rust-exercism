pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut i = 2;
    let mut factors: Vec<u64> = Vec::new();
    while n > 1 {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 1
    }
    factors
}

// Slow to run, fun to implement.
// struct Prime {
//     primes: Vec<u64>
// }

// impl Prime {
//     pub fn new() -> Prime {
//         Prime { primes: Vec::new() }
//     }
// }

// impl Iterator for Prime {
//     type Item = u64;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.primes.len() {
//             0 => self.primes.push(2),
//             1 => self.primes.push(3),
//             2 => self.primes.push(5),
//             3 => self.primes.push(7),
//             _ => {
//                 let mut i = self.primes[self.primes.len()-1];
//                 loop {
//                     if !(i % 2 == 0 || i % 3 == 0 || i % 5 == 0 || i % 7 == 0) {
//                         if !self.primes.iter().any(|x| i % x == 0) {
//                             self.primes.push(i);
//                             break;
//                         }
//                     }
//                     i += 2;
//                 }
//             }
//         }
//         Some(self.primes[self.primes.len()-1])
//     }
// }

// pub fn factors(n: u64) -> Vec<u64> {
//     let mut n = n;
//     let mut factors: Vec<u64> = Vec::new();
//     for p in Prime::new() {
//         while n % p == 0 {
//             n /= p;
//             factors.push(p);
//         }
//         if n == 1 {
//             break;
//         }
//     }
//     factors
// }
