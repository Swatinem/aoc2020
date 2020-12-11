use crate::utils::{example, input};

mod day_08 {
    use crate::utils::split;

    #[derive(Debug)]
    enum Instruction {
        Nop(isize),
        Acc(isize),
        Jmp(isize),
    }

    #[derive(Debug, Default)]
    struct ProcessorState {
        instruction_pointer: usize,
        accumulator: isize,
    }

    impl ProcessorState {
        fn execute_instruction(&mut self, instr: &Instruction) -> isize {
            match instr {
                Instruction::Nop(_) => 1,
                Instruction::Acc(arg) => {
                    self.accumulator += *arg;
                    1
                }
                Instruction::Jmp(arg) => *arg,
            }
        }

        fn run(&mut self, instructions: &[Instruction]) -> bool {
            let mut executed: Vec<_> = instructions.iter().map(|i| (i, false)).collect();

            while let Some((instr, executed)) = executed.get_mut(self.instruction_pointer) {
                if *executed {
                    break;
                }
                *executed = true;
                let offset = self.execute_instruction(instr);
                if offset < 0 {
                    if let Some(ip) = self.instruction_pointer.checked_sub(-offset as usize) {
                        self.instruction_pointer = ip;
                    } else {
                        return false;
                    }
                } else {
                    self.instruction_pointer += offset as usize;
                }
                if self.instruction_pointer == instructions.len() {
                    return true;
                }
            }
            false
        }
    }

    fn parse_instructions(input: &str) -> Vec<Instruction> {
        input
            .lines()
            .flat_map(|line| {
                let (instr, arg) = split(line, " ")?;
                let arg = arg.parse().ok()?;
                Some(match instr {
                    "nop" => Instruction::Nop(arg),
                    "acc" => Instruction::Acc(arg),
                    "jmp" => Instruction::Jmp(arg),
                    _ => {
                        return None;
                    }
                })
            })
            .collect()
    }

    pub fn a(input: &str) -> isize {
        let instructions: Vec<_> = parse_instructions(input);
        let mut state = ProcessorState::default();

        state.run(&instructions);
        state.accumulator
    }

    fn swap_nop_jmp(instr: &mut Instruction) {
        match instr {
            Instruction::Nop(arg) => *instr = Instruction::Jmp(*arg),
            Instruction::Jmp(arg) => *instr = Instruction::Nop(*arg),
            _ => {}
        }
    }

    pub fn b(input: &str) -> isize {
        let mut instructions: Vec<_> = parse_instructions(input);
        let mut state = ProcessorState::default();

        if state.run(&instructions) {
            return state.accumulator;
        }

        for i in 0..instructions.len() {
            state = ProcessorState::default();
            if !matches!(
                instructions.get(i),
                Some(Instruction::Nop(_)) | Some(Instruction::Jmp(_))
            ) {
                continue;
            }
            instructions.get_mut(i).map(swap_nop_jmp);
            if state.run(&instructions) {
                return state.accumulator;
            }
            instructions.get_mut(i).map(swap_nop_jmp);
        }
        0
    }
}

mod day_09 {
    fn has_sums_to(numbers: &[usize], needle: usize) -> bool {
        let mut numbers = Vec::from(numbers);
        numbers.sort_unstable();
        for num in numbers.iter().copied() {
            if num > needle / 2 {
                return false;
            }
            let pair = needle - num;
            if matches!(numbers.binary_search(&pair), Ok(_)) {
                return true;
            }
        }
        false
    }

    fn parse_numbers(input: &str) -> Vec<usize> {
        input.lines().filter_map(|line| line.parse().ok()).collect()
    }

    fn find_invalid(numbers: &[usize], window: usize) -> usize {
        for window in numbers.windows(window + 1) {
            let needle = window.last().copied().unwrap_or(0);
            if !has_sums_to(&window[..window.len() - 1], needle) {
                return needle;
            }
        }
        0
    }

    pub fn a(input: &str, window: usize) -> usize {
        find_invalid(&parse_numbers(input), window)
    }

    pub fn b(input: &str, window: usize) -> usize {
        let numbers = parse_numbers(input);
        let invalid = find_invalid(&numbers, window);
        let mut numbers_slice = numbers.as_slice();
        while !numbers_slice.is_empty() {
            let mut sum = 0;
            let mut smallest = std::usize::MAX;
            let mut largest = 0;
            for n in numbers_slice.iter().copied() {
                smallest = smallest.min(n);
                largest = largest.max(n);
                sum += n;
                if sum == invalid {
                    return smallest + largest;
                }
                if sum > invalid {
                    break;
                }
            }
            numbers_slice = &numbers_slice[1..];
        }
        0
    }
}

mod day_10 {
    use std::collections::HashMap;

    fn parse_sorted_numbers(input: &str) -> Vec<usize> {
        let mut numbers: Vec<_> = input.lines().filter_map(|line| line.parse().ok()).collect();
        numbers.push(0);
        numbers.sort_unstable();
        numbers.push(numbers.last().copied().unwrap_or(0) + 3);
        numbers
    }

    pub fn a(input: &str) -> usize {
        let numbers = parse_sorted_numbers(input);

        let mut diff1 = 0;
        let mut diff3 = 0;
        let mut last = 0;
        for num in numbers.into_iter() {
            match num - last {
                1 => diff1 += 1,
                3 => diff3 += 1,
                _ => {}
            }
            last = num;
        }

        diff1 * diff3
    }

    /// I cheated :-( Took inspiration from:
    /// <https://dev.to/qviper/advent-of-code-2020-python-solution-day-10-30kd>
    pub fn b(input: &str) -> usize {
        let numbers = parse_sorted_numbers(input);
        let mut map = HashMap::new();
        map.insert(0, 1);
        let last = numbers.last().copied().unwrap_or(0);
        for num in numbers.into_iter().skip(1) {
            let mut possible_predecessors = 0;
            possible_predecessors += map.get(&num.wrapping_sub(1)).copied().unwrap_or(0);
            possible_predecessors += map.get(&num.wrapping_sub(2)).copied().unwrap_or(0);
            possible_predecessors += map.get(&num.wrapping_sub(3)).copied().unwrap_or(0);
            map.insert(num, possible_predecessors);
        }
        map.get(&last).copied().unwrap_or(0)
    }
}

mod day_11 {
    use std::fmt::Display;

    #[derive(Copy, Clone, Debug)]
    struct Coord {
        row: usize,
        column: usize,
    }

    #[derive(Clone, Debug, PartialEq)]
    struct Matrix<T> {
        rows: usize,
        columns: usize,
        data: Vec<T>,
    }

    impl<T> Display for Matrix<T>
    where
        T: Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for row in 0..self.rows {
                for item in &self.data[(row * self.columns)..((row + 1) * self.columns)] {
                    write!(f, "{}", item)?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }

    impl<T> Matrix<T> {
        pub fn from_column_vec(data: Vec<T>, columns: usize) -> Option<Self> {
            let len = data.len();
            if len % columns != 0 {
                return None;
            }
            Some(Self {
                rows: len / columns,
                columns,
                data,
            })
        }

        pub fn get(&self, Coord { row, column }: Coord) -> Option<&T> {
            if column >= self.columns {
                return None;
            }
            self.data.get(row * self.columns + column)
        }

        fn trace_direction<F>(
            &self,
            coord: Coord,
            row_dir: isize,
            col_dir: isize,
            filter: &F,
        ) -> Option<&T>
        where
            F: Fn(&T) -> bool,
        {
            let Coord {
                mut row,
                mut column,
            } = coord;

            loop {
                row = match (row, row_dir) {
                    (0, -1) => return None,
                    (n, 1) if n == self.rows - 1 => return None,
                    (n, -1) => n - 1,
                    (n, 1) => n + 1,
                    (n, _) => n,
                };
                column = match (column, col_dir) {
                    (0, -1) => return None,
                    (n, 1) if n == self.columns - 1 => return None,
                    (n, -1) => n - 1,
                    (n, 1) => n + 1,
                    (n, _) => n,
                };
                let val = self.get(Coord { row, column })?;

                if filter(val) {
                    return Some(val);
                }
            }
        }

        pub fn neighbors<F>(&self, coord: Coord, filter: F) -> [Option<&T>; 8]
        where
            F: Fn(&T) -> bool,
        {
            [
                self.trace_direction(coord, -1, -1, &filter),
                self.trace_direction(coord, -1, 0, &filter),
                self.trace_direction(coord, -1, 1, &filter),
                self.trace_direction(coord, 0, -1, &filter),
                // Nope: self.trace_direction(coord, 0, 0, &filter),
                self.trace_direction(coord, 0, 1, &filter),
                self.trace_direction(coord, 1, -1, &filter),
                self.trace_direction(coord, 1, 0, &filter),
                self.trace_direction(coord, 1, 1, &filter),
            ]
        }

        pub fn coords(&self) -> impl Iterator<Item = Coord> + '_ {
            let columns = self.columns;
            (0..self.data.len()).map(move |i| Coord {
                row: i / columns,
                column: i % columns,
            })
        }

        pub fn mut_values(&mut self) -> impl Iterator<Item = &mut T> + '_ {
            self.data.iter_mut()
        }
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum Seat {
        Floor,
        Empty,
        Occupied,
    }

    impl Display for Seat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Seat::Floor => '.',
                    Seat::Empty => 'L',
                    Seat::Occupied => '#',
                }
            )
        }
    }

    fn parse_layout(input: &str) -> Matrix<Seat> {
        let mut data = vec![];
        let mut columns = 0;
        for line in input.lines() {
            columns = line.len();
            data.extend(line.chars().filter_map(|c| match c {
                '.' => Some(Seat::Floor),
                'L' => Some(Seat::Empty),
                '#' => Some(Seat::Occupied),
                _ => None,
            }))
        }
        Matrix::from_column_vec(data, columns).unwrap()
    }

    fn simulate_a(mut start: Matrix<Seat>) -> Matrix<Seat> {
        let mut simulated = start.clone();

        loop {
            // println!("{}", start);

            for (coord, new_seat) in start.coords().zip(simulated.mut_values()) {
                let seat = *start.get(coord).unwrap();
                if seat == Seat::Floor {
                    continue;
                }
                let neighbors = start.neighbors(coord, |_| true);
                let occupied = neighbors
                    .iter()
                    .filter_map(|o| *o)
                    .filter(|seat| **seat == Seat::Occupied)
                    .count();

                let seat_decision = if seat == Seat::Empty && occupied == 0 {
                    Seat::Occupied
                } else if seat == Seat::Occupied && occupied >= 4 {
                    Seat::Empty
                } else {
                    seat
                };

                *new_seat = seat_decision;
            }

            if simulated == start {
                return simulated;
            }
            std::mem::swap(&mut simulated, &mut start);
        }
    }

    fn simulate_b(mut start: Matrix<Seat>) -> Matrix<Seat> {
        let mut simulated = start.clone();

        loop {
            // println!("{}", start);

            for (coord, new_seat) in start.coords().zip(simulated.mut_values()) {
                let seat = *start.get(coord).unwrap();
                if seat == Seat::Floor {
                    continue;
                }
                let neighbors = start.neighbors(coord, |seat| *seat != Seat::Floor);
                let occupied = neighbors
                    .iter()
                    .filter_map(|o| *o)
                    .filter(|seat| **seat == Seat::Occupied)
                    .count();

                let seat_decision = if seat == Seat::Empty && occupied == 0 {
                    Seat::Occupied
                } else if seat == Seat::Occupied && occupied >= 5 {
                    Seat::Empty
                } else {
                    seat
                };

                *new_seat = seat_decision;
            }

            if simulated == start {
                return simulated;
            }
            std::mem::swap(&mut simulated, &mut start);
        }
    }

    pub fn a(input: &str) -> usize {
        let layout = parse_layout(input);
        let mut layout = simulate_a(layout);
        layout
            .mut_values()
            .filter(|seat| **seat == Seat::Occupied)
            .count()
    }

    pub fn b(input: &str) -> usize {
        let layout = parse_layout(input);
        let mut layout = simulate_b(layout);
        layout
            .mut_values()
            .filter(|seat| **seat == Seat::Occupied)
            .count()
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(day_08::a(&example("08")?), 5);
    assert_eq!(day_08::b(&example("08")?), 8);

    println!("day 08 a: {:?}", day_08::a(&input("08")?));
    println!("day 08 b: {:?}", day_08::b(&input("08")?));

    assert_eq!(day_09::a(&example("09")?, 5), 127);
    assert_eq!(day_09::b(&example("09")?, 5), 62);

    println!("day 09 a: {:?}", day_09::a(&input("09")?, 25));
    println!("day 09 b: {:?}", day_09::b(&input("09")?, 25));

    assert_eq!(day_10::a(&example("10a1")?), 35);
    assert_eq!(day_10::a(&example("10a2")?), 220);
    assert_eq!(day_10::b(&example("10a1")?), 8);
    assert_eq!(day_10::b(&example("10a2")?), 19208);

    println!("day 10 a: {:?}", day_10::a(&input("10")?));
    println!("day 10 b: {:?}", day_10::b(&input("10")?));

    assert_eq!(day_11::a(&example("11")?), 37);
    assert_eq!(day_11::b(&example("11")?), 26);

    println!("day 11 a: {:?}", day_11::a(&input("11")?));
    println!("day 11 b: {:?}", day_11::b(&input("11")?));
    Ok(())
}
