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

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(day_08::a(&example("08")?), 5);
    assert_eq!(day_08::b(&example("08")?), 8);

    println!("day 08 a: {:?}", day_08::a(&input("08")?));
    println!("day 08 b: {:?}", day_08::b(&input("08")?));

    Ok(())
}
