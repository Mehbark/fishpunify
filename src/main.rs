mod parse;
use itertools::Itertools;
use parse::puns;

fn main() {
    let mut input = std::env::args().skip(1).join(" ");
    for (find, replace) in puns() {
        input = input.replace(find, replace);
    }
    println!("{input}");
}
