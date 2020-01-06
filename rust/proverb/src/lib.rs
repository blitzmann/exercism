pub fn build_proverb(list: &[&str]) -> String {
    let v: Vec<str> = Vec::new();
    let mut index = 0;
    while index < list.len() {
        if (index == list.len() - 1) {
            v.push(format!("For want of a {} the {} was lost.", list[index], list[index+1])
        } else {
            v.push(format!("And all for the want of a {}}.", list[0])
        }
        index += 1;
    }
    return v.join('\n');
}
