

pub fn verse(n: u32) -> String {

    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => {
            format!("{0} bottle{1} of beer on the wall, {0} bottle{1} of beer.\nTake {2} down and pass it around, {3}{4} bottle{5} of beer on the wall.\n",
                    &n.to_string(), "", "it", "", "no more", "s")
        }
        2 => {
            format!("{0} bottle{1} of beer on the wall, {0} bottle{1} of beer.\nTake {2} down and pass it around, {3}{4} bottle{5} of beer on the wall.\n",
                    &n.to_string(), "s", "one", "", &(n-1).to_string(), "")
        }
        v => {
            format!("{0} bottle{1} of beer on the wall, {0} bottle{1} of beer.\nTake {2} down and pass it around, {3}{4} bottle{5} of beer on the wall.\n",
                    &v.to_string(), "s", "one", "", &(v-1).to_string(), "s")
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut out = String::new();
    for i in (end..=start).rev() {
        out.push_str(&verse(i));
        if i != end {
            out.push_str("\n");
        }
    }
    out
}

