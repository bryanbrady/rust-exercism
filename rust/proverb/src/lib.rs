
pub fn line(a: &str, b: &str) -> String {
    format!("For want of a {0} the {1} was lost.", a, b)
}

pub fn last(a: &str) -> String{
    format!("And all for the want of a {0}.", a)
}

pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::from(""),
        1 => last(list[0]),
        _ => {
            let mut out = String::new();
            for i in 0..(list.len()-1) {
                out.push_str(&line(list[i], list[i+1])[..]);
                out.push_str("\n");
            }
            out.push_str(&last(list[0])[..]);
            out
        }
    }
}
