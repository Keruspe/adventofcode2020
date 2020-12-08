#![feature(str_split_once)]

use std::sync::atomic::{AtomicI64, Ordering};

static INPUT: &str = include_str!("./08.txt");
static ACCUMULATOR: AtomicI64 = AtomicI64::new(0);

#[derive(Clone)]
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

fn run(instructions: &[Instruction]) -> bool {
    ACCUMULATOR.store(0, Ordering::SeqCst);
    let mut passed_instructions = instructions.iter().map(|_| false).collect::<Vec<_>>();
    let mut idx = 0usize;
    loop {
        if idx == instructions.len() {
            println!("Program terminated with accumulator {}", ACCUMULATOR.load(Ordering::SeqCst));
            return true;
        }
        if passed_instructions[idx] {
            println!("Infinite loop at idx {} with accumulator {}, aborting", idx, ACCUMULATOR.load(Ordering::SeqCst));
            return false;
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
    let mut attempted_nops = instructions.iter().enumerate().filter_map(|(idx, ins)| if let Instruction::Nop(arg) = ins { Some((idx, false, arg)) } else { None }).collect::<Vec<_>>();
    let mut attempted_jmps = instructions.iter().enumerate().filter_map(|(idx, ins)| if let Instruction::Jmp(_) = ins { Some((idx, false)) } else { None }).collect::<Vec<_>>();
    loop {
        let mut instructions = instructions.clone();
        if let Some((i, idx, arg)) = attempted_nops.clone().iter().enumerate().filter_map(|(i, (idx, b, arg))| if !*b { Some((i, idx, arg)) } else { None }).next() {
            attempted_nops[i].1 = true;
            instructions[*idx] = Instruction::Jmp(**arg);
        } else if let Some((i, idx)) = attempted_jmps.clone().iter().enumerate().filter_map(|(i, (idx, b))| if !*b { Some((i, idx)) } else { None }).next() {
            attempted_jmps[i].1 = true;
            instructions[*idx] = Instruction::Nop(0);
        } else {
            unreachable!();
        }
        if run(&instructions) {
            break;
        }
    }
}
