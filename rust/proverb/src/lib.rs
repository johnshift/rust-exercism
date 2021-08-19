pub fn build_proverb(list: &[&str]) -> String {
    let mut lines: Vec<String> = Vec::new();
    let length = list.len();

    // empty list
    if length == 0 {
        return String::new();
    }

    for (i, _) in list.into_iter().enumerate() {
        // last element
        if i == length - 1 {
            lines.push(format!("And all for the want of a {}.", list[0]));
        } else {
            lines.push(format!(
                "For want of a {} the {} was lost.",
                list[i],
                list[i + 1]
            ));
        }
    }

    lines.join("\n")
}
