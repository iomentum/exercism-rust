pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = "".to_string();

    if list.len() == 0 {
        return proverb;
    }

    for i in 0..list.len() - 1 {
        proverb.push_str(&format!("For want of a {} the {} was lost.\n", list[i], list[i+1]));
    }

    proverb.push_str(&format!("And all for the want of a {}.", list[0]));

    proverb
}
