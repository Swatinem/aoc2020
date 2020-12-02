mod day_01 {
    use std::collections::HashSet;

    /// Reads a list of numbers from the input text.
    fn read_numbers(input: &str) -> HashSet<usize> {
        input
            .lines()
            .filter_map(|l| l.parse::<usize>().ok())
            .collect()
    }

    /// Finds two numbers in the input list that sum to the given number.
    fn find_sums_to(numbers: &HashSet<usize>, sum: usize) -> Option<(usize, usize)> {
        for first_num in numbers.iter().copied().filter(|n| *n < sum) {
            let second_num = sum - first_num;
            if numbers.contains(&second_num) {
                return Some((first_num, second_num));
            }
        }
        None
    }

    pub fn a(input: &str) -> Option<usize> {
        let numbers = read_numbers(input);
        find_sums_to(&numbers, 2020).map(|(first_num, second_num)| first_num * second_num)
    }

    pub fn b(input: &str) -> Option<usize> {
        let numbers = read_numbers(input);

        for first_num in numbers.iter().copied() {
            if let Some((second_num, third_num)) = find_sums_to(&numbers, 2020 - first_num) {
                return Some(first_num * second_num * third_num);
            }
        }
        None
    }
}

mod day_02 {
    use std::collections::HashMap;

    #[derive(Debug)]
    struct PasswordExample {
        min_occurrences: usize,
        max_occurrences: usize,
        constrained_char: char,
        password: String,
    }

    fn split<'a>(input: &'a str, search: &str) -> Option<(&'a str, &'a str)> {
        let mut iter = input.splitn(2, search);
        let fst = iter.next();
        let snd = iter.next();
        fst.and_then(|fst| snd.map(|snd| (fst, snd)))
    }

    fn parse_passwords(input: &str) -> Vec<PasswordExample> {
        input
            .lines()
            .filter_map(|rest| {
                let (min_occ, rest) = split(rest, "-")?;
                let (max_occ, rest) = split(rest, " ")?;
                let (ch, password) = split(rest, ": ")?;
                Some(PasswordExample {
                    min_occurrences: min_occ.parse().ok()?,
                    max_occurrences: max_occ.parse().ok()?,
                    constrained_char: ch.chars().next()?,
                    password: password.to_owned(),
                })
            })
            .collect()
    }

    fn count_letters(input: &str) -> HashMap<char, usize> {
        let mut map = HashMap::new();
        for letter in input.chars() {
            map.entry(letter)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        map
    }

    pub fn a(input: &str) -> usize {
        let passwords = parse_passwords(input);
        passwords
            .iter()
            .filter(|p| {
                let letters = count_letters(&p.password);
                let occurences = letters.get(&p.constrained_char).copied().unwrap_or(0);
                occurences >= p.min_occurrences && occurences <= p.max_occurrences
            })
            .count()
    }

    pub fn b(input: &str) -> usize {
        let passwords = parse_passwords(input);
        passwords
            .iter()
            .filter(|p| {
                let mut letters = p.password.chars();
                (letters.clone().nth(p.min_occurrences - 1) == Some(p.constrained_char)) as usize
                    + (letters.nth(p.max_occurrences - 1) == Some(p.constrained_char)) as usize
                    == 1
            })
            .count()
    }
}

fn main() {
    assert_eq!(day_01::a(include_str!("./example-01.txt")), Some(514579));
    assert_eq!(day_01::b(include_str!("./example-01.txt")), Some(241861950));

    println!("day 01 a: {:?}", day_01::a(include_str!("./input-01.txt")));
    println!("day 01 b: {:?}", day_01::b(include_str!("./input-01.txt")));

    assert_eq!(day_02::a(include_str!("./example-02.txt")), 2);
    assert_eq!(day_02::b(include_str!("./example-02.txt")), 1);

    println!("day 02 a: {:?}", day_02::a(include_str!("./input-02.txt")));
    println!("day 02 b: {:?}", day_02::b(include_str!("./input-02.txt")));
}
