#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
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

fn is_sublist<T: PartialEq>(sub: &[T], main: &[T]) -> bool {
    // `windows(0)` is not allowed (will panic), so handle empty `sub` explicitly.
    if sub.is_empty() {
        return true;
    }
    if sub.len() > main.len() {
        return false;
    }
    main.windows(sub.len()).any(|x| x == sub)
}

