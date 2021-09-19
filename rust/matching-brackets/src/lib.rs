pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                match stack.last() {
                    Some(cc) => {
                        match (cc, c) {
                            ('(', ')') | ('[', ']') | ('{', '}') => stack.pop(),
                            _ => return false
                        }
                    },
                    None => return false
                };
            },
            _ => ()
        };
    }
    stack.is_empty()
}

