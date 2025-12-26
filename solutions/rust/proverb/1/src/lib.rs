pub fn build_proverb(list: &[&str]) -> String {
    // todo!("build a proverb from this list of items: {list:?}");
    if list.len() == 0 {
        return String::new();
    }
    if list.len() == 1 {
        return format!("And all for the want of a {}.", list[0]);
    }
    let mut proverb = String::new();
    for i in 0..list.len() - 1 {
        proverb.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            list[i], list[i + 1]
        ));
    }
    proverb.push_str(&format!("And all for the want of a {}.", list[0]));
    proverb
}
