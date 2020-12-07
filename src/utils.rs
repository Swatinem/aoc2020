use std::collections::HashMap;

pub fn split<'a>(input: &'a str, search: &str) -> Option<(&'a str, &'a str)> {
    let mut iter = input.splitn(2, search);
    let fst = iter.next();
    let snd = iter.next();
    fst.and_then(|fst| snd.map(|snd| (fst, snd)))
}

type InputResult<'s> = std::io::Result<&'s str>;

#[derive(Default)]
pub struct Inputs {
    cache: HashMap<String, String>,
}

impl Inputs {
    /// Creates a new Input Cache.
    pub fn new() -> Self {
        Self::default()
    }

    fn fetch(&mut self, ex: &str) -> std::io::Result<()> {
        let path = std::path::Path::new("inputs").join(ex);
        let contents = std::fs::read_to_string(path)?;
        self.cache.insert(ex.to_owned(), contents);
        Ok(())
    }

    fn get(&mut self, ex: &str) -> InputResult<'_> {
        if !self.cache.contains_key(ex) {
            self.fetch(ex)?;
        }
        Ok(self.cache.get(ex).map(|s| s.as_str()).unwrap_or(""))
    }

    /// Get the example input for the given day.
    pub fn example(&mut self, day: &str) -> InputResult<'_> {
        self.get(&format!("example-{}.txt", day))
    }

    /// Get the real input for the given day.
    pub fn input(&mut self, day: &str) -> InputResult<'_> {
        self.get(&format!("input-{}.txt", day))
    }
}
