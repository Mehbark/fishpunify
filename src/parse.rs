use std::collections::HashMap;

const PUNS: &str = include_str!("../pun_list.txt");
pub fn puns() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    for line in PUNS.lines() {
        if let Some((find, replace)) = line.split_once('=') {
            for item in find.split('/') {
                map.insert(item, replace);
            }
        }
    }

    map
}

pub fn punify(puns: &HashMap<&str, &str>, text: &str) -> String {
    let mut text = text.to_string();

    for (find, replace) in puns {
        text = text.replace(find, replace);
    }

    text
}
