// https://www.codewars.com/kata/5277c8a221e209d3f6000b56/train/rust

fn valid_braces(s: &str) -> bool {
    let mut braces = std::collections::HashMap::new();
    braces.insert('(', ')');
    braces.insert('{', '}');
    braces.insert('[', ']');
    let mut stack = Vec::new();
    for char in s.chars() {
        for (opening_brace, closing_brace) in &braces {
            if &char == opening_brace {
                stack.push(char);
            } else if &char == closing_brace {
                if stack.last().is_some() {
                    if stack.last().unwrap() == opening_brace {
                        stack.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    stack.len() == 0
}

pub fn verify() {
    assert_eq!(true, valid_braces("()"));
    assert_eq!(false, valid_braces("[(])"));
    assert_eq!(false, valid_braces("(((({{"));
}
