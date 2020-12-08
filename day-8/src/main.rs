use std::fs;

#[derive(Debug)]
enum Operation {
    NoOperation,
    Accumulator,
    Jump,
}
#[derive(Debug)]
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
