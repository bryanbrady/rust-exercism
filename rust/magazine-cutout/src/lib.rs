// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut mmap = HashMap::new();
    let mut nmap = HashMap::new();

    for word in magazine {
        *mmap.entry(word).or_insert(1_u32) += 1;
    }
    for word in note {
        *nmap.entry(word).or_insert(1_u32) += 1;
    }

    for n in note {
        let in_nmap = nmap.contains_key(n);
        if mmap.contains_key(n) != nmap.contains_key(n){
            return false
        } else if mmap[n] < nmap[n] {
            return false
        }
    }
    true
}
