fn split<'a>(input: &'a str, search: &str) -> Option<(&'a str, &'a str)> {
    let mut iter = input.splitn(2, search);
    let fst = iter.next();
    let snd = iter.next();
    fst.and_then(|fst| snd.map(|snd| (fst, snd)))
}

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

    pub fn a(input: &str) -> usize {
        let numbers = read_numbers(input);
        find_sums_to(&numbers, 2020)
            .map(|(first_num, second_num)| first_num * second_num)
            .unwrap_or_default()
    }

    pub fn b(input: &str) -> usize {
        let numbers = read_numbers(input);

        for first_num in numbers.iter().copied() {
            if let Some((second_num, third_num)) = find_sums_to(&numbers, 2020 - first_num) {
                return first_num * second_num * third_num;
            }
        }
        0
    }
}

mod day_02 {
    use std::collections::HashMap;

    use crate::split;

    #[derive(Debug)]
    struct PasswordExample {
        min_occurrences: usize,
        max_occurrences: usize,
        constrained_char: char,
        password: String,
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
                (letters.clone().nth(p.min_occurrences - 1) == Some(p.constrained_char))
                    != (letters.nth(p.max_occurrences - 1) == Some(p.constrained_char))
            })
            .count()
    }
}

mod day_03 {
    pub fn try_slope(rows: &[&str], (slope_col, slope_row): (usize, usize)) -> usize {
        let mut row = slope_row;
        let mut col = slope_col;
        let mut trees_hit = 0;
        while row < rows.len() {
            if let Some('#') = rows
                .get(row)
                .and_then(|row| row.chars().nth(col % row.len()))
            {
                trees_hit += 1;
            }
            row += slope_row;
            col += slope_col;
        }
        trees_hit
    }

    pub fn a(input: &str) -> usize {
        let rows: Vec<_> = input.lines().collect();
        try_slope(&rows, (3, 1))
    }

    pub fn b(input: &str) -> usize {
        let rows: Vec<_> = input.lines().collect();
        try_slope(&rows, (1, 1))
            * try_slope(&rows, (3, 1))
            * try_slope(&rows, (5, 1))
            * try_slope(&rows, (7, 1))
            * try_slope(&rows, (1, 2))
    }
}

mod day_04 {
    use std::collections::HashMap;

    use crate::split;

    const REQUIRED: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; // "cid"
    const ECL: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    fn parse_passports(input: &str) -> Vec<HashMap<&str, &str>> {
        let mut passports = vec![];
        let mut passport = HashMap::new();
        for line in input.lines() {
            if line.is_empty() {
                passports.push(std::mem::take(&mut passport));
            }
            for kv in line.split_whitespace() {
                if let Some((k, v)) = split(kv, ":") {
                    passport.insert(k, v);
                }
            }
        }
        passports.push(passport);
        passports
    }

    fn is_valid_a(pp: &HashMap<&str, &str>) -> bool {
        REQUIRED.iter().all(|k| pp.contains_key(k))
    }

    fn is_valid_b(pp: HashMap<&str, &str>) -> Option<bool> {
        let byr: usize = pp.get("byr")?.parse().ok()?;
        if byr < 1920 || byr > 2002 {
            return None;
        }
        let iyr: usize = pp.get("iyr")?.parse().ok()?;
        if iyr < 2010 || iyr > 2020 {
            return None;
        }
        let eyr: usize = pp.get("eyr")?.parse().ok()?;
        if eyr < 2020 || eyr > 2030 {
            return None;
        }

        let hgt = pp.get("hgt")?;
        if let Some(hgt) = hgt.strip_suffix("cm") {
            let cm: usize = hgt.parse().ok()?;
            if cm < 150 || cm > 193 {
                return None;
            }
        } else if let Some(hgt) = hgt.strip_suffix("in") {
            let inch: usize = hgt.parse().ok()?;
            if inch < 59 || inch > 76 {
                return None;
            }
        } else {
            return None;
        }

        let hcl = pp.get("hcl")?.strip_prefix("#")?;
        if hcl.len() != 6 || hcl.chars().any(|c| !c.is_digit(16)) {
            return None;
        }
        let ecl = pp.get("ecl")?;
        if !ECL.contains(ecl) {
            return None;
        }

        let pid = pp.get("pid")?;
        if pid.len() != 9 || pid.chars().any(|c| !c.is_numeric()) {
            return None;
        }

        Some(true)
    }

    pub fn a(input: &str) -> usize {
        parse_passports(input)
            .into_iter()
            .filter(is_valid_a)
            .count()
    }

    pub fn b(input: &str) -> usize {
        parse_passports(input)
            .into_iter()
            .filter_map(is_valid_b)
            .count()
    }
}

mod day_05 {
    use std::ops::Range;

    const ROWS: usize = 128;
    const COLUMNS: usize = 8;

    fn len(range: &Range<usize>) -> usize {
        range.end - range.start
    }

    fn decode_boarding_pass(input: &str) -> (usize, usize) {
        let mut row = 0..ROWS;
        let mut column = 0..COLUMNS;
        for c in input.chars() {
            match c {
                'F' => {
                    row.end -= len(&row) / 2;
                }
                'B' => {
                    row.start += len(&row) / 2;
                }
                'L' => {
                    column.end -= len(&column) / 2;
                }
                'R' => {
                    column.start += len(&column) / 2;
                }
                _ => unreachable!(),
            }
        }

        (row.start, column.start)
    }

    fn seat_id((row, column): (usize, usize)) -> usize {
        row * COLUMNS + column
    }

    pub fn a(input: &str) -> usize {
        input
            .lines()
            .map(decode_boarding_pass)
            .map(seat_id)
            .max()
            .unwrap_or_default()
    }

    pub fn b(input: &str) -> usize {
        let mut seats: Vec<_> = input
            .lines()
            .map(decode_boarding_pass)
            .map(seat_id)
            .collect();
        seats.sort_unstable();
        for win in seats.windows(2) {
            match win {
                [a, b] if b - a == 2 => {
                    return a + 1;
                }
                _ => {}
            }
        }
        0
    }
}

fn main() {
    assert_eq!(day_01::a(include_str!("./example-01.txt")), 514579);
    assert_eq!(day_01::b(include_str!("./example-01.txt")), 241861950);

    println!("day 01 a: {:?}", day_01::a(include_str!("./input-01.txt")));
    println!("day 01 b: {:?}", day_01::b(include_str!("./input-01.txt")));

    assert_eq!(day_02::a(include_str!("./example-02.txt")), 2);
    assert_eq!(day_02::b(include_str!("./example-02.txt")), 1);

    println!("day 02 a: {:?}", day_02::a(include_str!("./input-02.txt")));
    println!("day 02 b: {:?}", day_02::b(include_str!("./input-02.txt")));

    assert_eq!(day_03::a(include_str!("./example-03.txt")), 7);
    assert_eq!(day_03::b(include_str!("./example-03.txt")), 336);

    println!("day 03 a: {:?}", day_03::a(include_str!("./input-03.txt")));
    println!("day 03 b: {:?}", day_03::b(include_str!("./input-03.txt")));

    assert_eq!(day_04::a(include_str!("./example-04.txt")), 2);

    println!("day 04 a: {:?}", day_04::a(include_str!("./input-04.txt")));
    println!("day 04 b: {:?}", day_04::b(include_str!("./input-04.txt")));

    assert_eq!(day_05::a(include_str!("./example-05.txt")), 820);

    println!("day 05 a: {:?}", day_05::a(include_str!("./input-05.txt")));
    println!("day 05 b: {:?}", day_05::b(include_str!("./input-05.txt")));
}
