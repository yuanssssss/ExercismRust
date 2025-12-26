#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if is_sublist(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }

}

fn is_sublist(sub: &[i32], main: &[i32]) -> bool {
    if sub.is_empty() {
        return true;
    }
    if sub.len() > main.len() {
        return false;
    }

    for i in 0..=(main.len() - sub.len()) {
        if &main[i..i + sub.len()] == sub {
            return true;
        }
    }
    false
}

