pub fn split<'a>(input: &'a str, search: &str) -> Option<(&'a str, &'a str)> {
    let mut iter = input.splitn(2, search);
    let fst = iter.next();
    let snd = iter.next();
    fst.and_then(|fst| snd.map(|snd| (fst, snd)))
}

fn get(ex: &str) -> std::io::Result<String> {
    let path = std::path::Path::new("inputs").join(ex);
    std::fs::read_to_string(path)
}

/// Get the example input for the given day.
pub fn example(day: &str) -> std::io::Result<String> {
    get(&format!("example-{}.txt", day))
}

/// Get the real input for the given day.
pub fn input(day: &str) -> std::io::Result<String> {
    get(&format!("input-{}.txt", day))
}
