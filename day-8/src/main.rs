use std::fs;
use std::mem;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    NoOperation,
    Accumulator,
    Jump,
}
#[derive(Debug, PartialEq, Copy, Clone)]
struct Instruction {
    executed: bool,
    operation: Operation,
    argument: i32,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let mut parts = line.split(" ");
        let operation = match parts.next().unwrap() {
            "acc" => Operation::Accumulator,
            "jmp" => Operation::Jump,
            "nop" => Operation::NoOperation,
            _ => Operation::NoOperation,
        };
        let argument = parts.next().unwrap().parse::<i32>().unwrap();

        Self {
            executed: false,
            operation,
            argument,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let instructions: Vec<Instruction> = lines.map(|line| Instruction::new(line)).collect();

    println!("{:?}", instructions);

    let jumps_or_nops: Vec<(usize, Instruction)> = instructions
        .iter()
        .enumerate()
        .filter_map(|(index, instruction)| {
            if instruction.operation == Operation::Jump
                || instruction.operation == Operation::NoOperation
            {
                return Some((index, instruction.clone()));
            }
            None
        })
        .collect();

    println!("{:?}", jumps_or_nops);

    // For each item that can be edited, check if it's possible to terminate the program.
    jumps_or_nops.iter().for_each(|(index, _)| {
        // replace instruction in list
        let mut instructions_edited = instructions.clone();
        let edited_instruction = instructions_edited.get_mut(index.clone()).unwrap();

        if edited_instruction.operation == Operation::NoOperation {
            edited_instruction.operation = Operation::Jump;
        }

        if edited_instruction.operation == Operation::Jump {
            edited_instruction.operation = Operation::NoOperation;
        }

        let result = terminate_program(instructions_edited);

        if result.is_ok() {
            println!("{:?}", result);
        }

    });
}

fn terminate_program(mut instructions: Vec<Instruction>) -> Result<i32, &'static str> {
    let instructions_total_count = instructions.iter().count();

    let mut index: i32 = 0;
    let mut accumulator = 0;
    loop {
        let instruction = instructions.get_mut(index as usize);

        match instruction {
            Some(i) => {
                // if second time it means it's an infinite loop and it's the wrong value that has been modified
                if i.executed {
                    break (Err("Loop"));
                }

                match i.operation {
                    Operation::Accumulator => {
                        accumulator += i.argument;
                        index += 1;
                    }
                    Operation::Jump => {
                        index += i.argument;
                    }
                    Operation::NoOperation => {
                        index += 1;
                    }
                }

                i.executed = true;
            }
            None => {
                // end of the program
                break (Ok(accumulator));
            }
        }
        // println!("{:?}", instruction);
    }
}

fn part_one() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();

    let mut instructions: Vec<Instruction> = lines.map(|line| Instruction::new(line)).collect();

    println!("{:?}", instructions);

    let instructions_total_count = instructions.iter().count();
    println!("{:?}", instructions_total_count);

    let mut index: i32 = 0;
    let mut accumulator = 0;
    loop {
        let instruction = instructions.get_mut(index as usize).unwrap();
        println!("{:?}", instruction);

        // if second time it's executed, stop
        if instruction.executed {
            break;
        }

        match instruction.operation {
            Operation::Accumulator => {
                accumulator += instruction.argument;
                index += 1;
            }
            Operation::Jump => {
                index += instruction.argument;
            }
            Operation::NoOperation => {
                index += 1;
            }
        }

        instruction.executed = true;
    }

    println!("{}", accumulator);
}
