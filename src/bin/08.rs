#![feature(str_split_once)]

use std::sync::atomic::{AtomicI64, Ordering};

static INPUT: &str = include_str!("./08.txt");
static ACCUMULATOR: AtomicI64 = AtomicI64::new(0);

enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

fn parse_arg(s: &str) -> i64 {
    let (sign, value) = s.split_at(1);
    let value = value.parse().unwrap();
    match sign {
        "-" => -1 * value,
        "+" => value,
        _ => unreachable!(),
    }
}

fn parse_instruction(s: &str) -> Instruction {
    let (instr, arg) = s.split_once(" ").unwrap();
    let arg = parse_arg(arg);
    match instr {
        "acc" => Instruction::Acc(arg),
        "jmp" => Instruction::Jmp(arg),
        "nop" => Instruction::Nop(arg),
        _ => unreachable!(),
    }
}

fn run(instructions: &[Instruction]) {
    let mut passed_instructions = instructions.iter().map(|_| false).collect::<Vec<_>>();
    let mut idx = 0usize;
    loop {
        if idx == instructions.len() {
            println!("Program terminated with accumulator {}", ACCUMULATOR.load(Ordering::SeqCst));
            break;
        }
        if passed_instructions[idx] {
            println!("Infinite loop at idx {} with accumulator {}, aborting", idx, ACCUMULATOR.load(Ordering::SeqCst));
            break;
        }
        passed_instructions[idx] = true;
        match instructions[idx] {
            Instruction::Acc(arg) => {
                ACCUMULATOR.fetch_add(arg, Ordering::SeqCst);
                idx += 1;
            },
            Instruction::Jmp(arg) => idx = ((idx as i64) + arg) as usize,
            Instruction::Nop(_) => idx += 1,
        }
    }
}

fn main() {
    let instructions = INPUT.lines().map(parse_instruction).collect::<Vec<_>>();
    run(&instructions);
}
