pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|x| factors.iter().any(|f| *f != 0 && x % f == 0)).sum()
}

// pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
//     let mut out: Vec<u32> = factors
//         .iter()
//         .filter(|x| **x != 0)
//         .map(|&x| (x..)
//             .step_by(x as usize)
//             .take_while(|&y| y < limit)
//             .collect::<Vec<u32>>())
//         .collect::<Vec<Vec<u32>>>()
//         .concat();
//     out.sort();
//     out.dedup();
//     out.iter().sum()
// }
