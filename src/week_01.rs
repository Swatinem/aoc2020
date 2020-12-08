use crate::utils::{example, input};

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

    use crate::utils::split;

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

    use crate::utils::split;

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

mod day_06 {
    use std::collections::{HashMap, HashSet};

    pub fn a(input: &str) -> usize {
        let mut group = HashSet::new();
        let mut sum = 0;

        for line in input.lines() {
            if line.is_empty() {
                sum += group.len();
                group.clear();
            }

            for c in line.chars() {
                group.insert(c);
            }
        }
        sum += group.len();

        sum
    }

    pub fn count_matching(map: &HashMap<char, usize>, needle: usize) -> usize {
        map.values().copied().filter(|v| *v == needle).count()
    }

    pub fn b(input: &str) -> usize {
        let mut group = HashMap::new();
        let mut sum = 0;
        let mut count = 0;

        for line in input.lines() {
            if line.is_empty() {
                sum += count_matching(&group, count);
                count = 0;
                group.clear();
                continue;
            }

            for c in line.chars() {
                group.entry(c).and_modify(|v| *v += 1).or_insert(1);
            }
            count += 1;
        }
        sum += count_matching(&group, count);

        sum
    }
}

mod day_07 {
    use std::collections::HashMap;

    use crate::utils::split;

    #[derive(Debug)]
    struct Bag {
        color: String,
        children: HashMap<String, usize>,
    }

    fn parse_bag(input: &str) -> Option<Bag> {
        let (color, mut rest) = split(input, " bags contain ")?;
        let mut bag = Bag {
            color: color.to_owned(),
            children: HashMap::new(),
        };

        if rest != "no other bags." {
            loop {
                let (num, rest2) = split(rest, " ")?;
                let num = num.parse().ok()?;
                let (color, mut rest2) = split(rest2, " bag")?;
                bag.children.insert(color.to_owned(), num);
                if let Some(rest3) = rest2.strip_prefix("s") {
                    rest2 = rest3;
                }
                if let Some(rest3) = rest2.strip_prefix(", ") {
                    rest = rest3;
                } else {
                    break;
                }
            }
        }

        Some(bag)
    }

    fn parse_bags(input: &str) -> HashMap<String, Bag> {
        let mut bags = HashMap::new();
        for line in input.lines() {
            if let Some(bag) = parse_bag(line) {
                bags.insert(bag.color.clone(), bag);
            }
        }
        bags
    }

    fn does_contain(
        bags: &HashMap<String, Bag>,
        cache: &mut HashMap<String, bool>,
        id: &str,
        needle: &str,
    ) -> bool {
        if let Some(contains) = cache.get(id) {
            return *contains;
        }
        if let Some(bag) = bags.get(id) {
            let does_contain = bag
                .children
                .iter()
                .any(|(id, _)| id == needle || does_contain(bags, cache, id, needle));
            cache.insert(id.to_owned(), does_contain);
            return does_contain;
        }
        false
    }

    pub fn a(input: &str) -> usize {
        let bags = parse_bags(input);
        let mut cache = HashMap::new();
        bags.keys()
            .filter(|color| does_contain(&bags, &mut cache, color.as_ref(), "shiny gold"))
            .count()
    }

    fn total_count(
        bags: &HashMap<String, Bag>,
        cache: &mut HashMap<String, usize>,
        id: &str,
    ) -> usize {
        if let Some(contains) = cache.get(id) {
            return *contains;
        }
        if let Some(bag) = bags.get(id) {
            let child_count = bag
                .children
                .iter()
                .map(|(id, num)| *num * (1 + total_count(bags, cache, id)))
                .sum();
            cache.insert(id.to_owned(), child_count);
            return child_count;
        }
        0
    }

    pub fn b(input: &str) -> usize {
        let bags = parse_bags(input);
        let mut cache = HashMap::new();
        total_count(&bags, &mut cache, "shiny gold")
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(day_01::a(&example("01")?), 514579);
    assert_eq!(day_01::b(&example("01")?), 241861950);

    println!("day 01 a: {:?}", day_01::a(&input("01")?));
    println!("day 01 b: {:?}", day_01::b(&input("01")?));

    assert_eq!(day_02::a(&example("02")?), 2);
    assert_eq!(day_02::b(&example("02")?), 1);

    println!("day 02 a: {:?}", day_02::a(&input("02")?));
    println!("day 02 b: {:?}", day_02::b(&input("02")?));

    assert_eq!(day_03::a(&example("03")?), 7);
    assert_eq!(day_03::b(&example("03")?), 336);

    println!("day 03 a: {:?}", day_03::a(&input("03")?));
    println!("day 03 b: {:?}", day_03::b(&input("03")?));

    assert_eq!(day_04::a(&example("04")?), 2);

    println!("day 04 a: {:?}", day_04::a(&input("04")?));
    println!("day 04 b: {:?}", day_04::b(&input("04")?));

    assert_eq!(day_05::a(&example("05")?), 820);

    println!("day 05 a: {:?}", day_05::a(&input("05")?));
    println!("day 05 b: {:?}", day_05::b(&input("05")?));

    assert_eq!(day_06::a(&example("06")?), 11);
    assert_eq!(day_06::b(&example("06")?), 6);

    println!("day 06 a: {:?}", day_06::a(&input("06")?));
    println!("day 06 b: {:?}", day_06::b(&input("06")?));

    assert_eq!(day_07::a(&example("07")?), 4);
    assert_eq!(day_07::b(&example("07b")?), 126);

    println!("day 07 a: {:?}", day_07::a(&input("07")?));
    println!("day 07 b: {:?}", day_07::b(&input("07")?));

    Ok(())
}
