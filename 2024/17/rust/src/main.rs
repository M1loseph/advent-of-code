use std::fs::read_to_string;

trait StdOutput {
    fn out(&mut self, out: u8);
}

struct VecStdOutput {
    vec: Vec<u8>,
}

impl VecStdOutput {
    fn new() -> Self {
        VecStdOutput { vec: Vec::new() }
    }
    fn output(&self) -> String {
        self.vec
            .iter()
            .map(|number| number.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl StdOutput for VecStdOutput {
    fn out(&mut self, out: u8) {
        self.vec.push(out);
    }
}

#[derive(Clone)]
struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_pointer: usize,
    program: Vec<u8>,
}

impl Computer {
    fn literal_operand(&self) -> i64 {
        let operand = self.program[self.instruction_pointer + 1];
        match operand {
            0..8 => operand as i64,
            _ => panic!("Unsupported operand value for combo operator: {operand}"),
        }
    }

    fn combo_operand(&self) -> i64 {
        let operand = self.program[self.instruction_pointer + 1];
        match operand {
            0..4 => operand as i64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Unsupported operand value for combo operator: {operand}"),
        }
    }

    fn execute<T>(&mut self, output: &mut T)
    where
        T: StdOutput,
    {
        while self.instruction_pointer < self.program.len() {
            let instruction = self.program[self.instruction_pointer];
            match instruction {
                0 => {
                    let numberator = self.register_a;
                    let denumerator = 2i64.pow(self.combo_operand() as u32);
                    self.register_a = numberator / denumerator;
                    self.instruction_pointer += 2;
                }
                1 => {
                    let result = self.register_b ^ self.literal_operand();
                    self.register_b = result;
                    self.instruction_pointer += 2;
                }
                2 => {
                    self.register_b = self.combo_operand() % 8;
                    self.instruction_pointer += 2;
                }
                3 => {
                    if self.register_a != 0 {
                        self.instruction_pointer = self.literal_operand() as usize;
                    } else {
                        self.instruction_pointer += 2;
                    }
                }
                4 => {
                    self.register_b = self.register_b ^ self.register_c;
                    self.instruction_pointer += 2;
                }
                5 => {
                    let out = self.combo_operand() % 8;
                    output.out(out as u8);
                    self.instruction_pointer += 2;
                }
                6 => {
                    let numberator = self.register_a;
                    let denumerator = 2i64.pow(self.combo_operand() as u32);
                    self.register_b = numberator / denumerator;
                    self.instruction_pointer += 2;
                }
                7 => {
                    let numberator = self.register_a;
                    let denumerator = 2i64.pow(self.combo_operand() as u32);
                    self.register_c = numberator / denumerator;
                    self.instruction_pointer += 2;
                }
                _ => {
                    panic!("Unexpected instruction {}", instruction);
                }
            }
        }
    }
}

fn puzzle_1(mut computer: Computer) {
    let mut output = VecStdOutput::new();
    computer.execute(&mut output);
    println!("Computer execution result: {}", output.output())
}

fn puzzle_2(mut computer: Computer) {
    fn ends_with(left: &Vec<u8>, right: &Vec<u8>) -> bool {
        if right.len() == 0 {
            return false;
        }
        &left[left.len() - right.len()..left.len()] == &right[..]
    }

    fn find_recursive(a_register: i64, computer: &mut Computer) -> Option<i64> {
        for i in 0..8 {
            let next_a_register = a_register + i;
            let mut out = VecStdOutput::new();
            computer.register_a = next_a_register;
            computer.register_b = 0;
            computer.register_c = 0;
            computer.instruction_pointer = 0;
            computer.execute(&mut out);
            if out.vec == computer.program {
                return Some(next_a_register);
            }
            if ends_with(&computer.program, &out.vec) {
                if let Some(value) = find_recursive(next_a_register * 8, computer) {
                    return Some(value);
                };
            }
        }
        None
    }
    let a_register = find_recursive(1, &mut computer);
    println!("A register value making program copiable is {:?}", a_register.unwrap());
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let registers = file_content
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    let register_a = registers[0]
        .chars()
        .skip("Register A: ".len())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let register_b = registers[1]
        .chars()
        .skip("Register B: ".len())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let register_c = registers[2]
        .chars()
        .skip("Register C: ".len())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let program = {
        let (_, instructions) = file_content
            .lines()
            .last()
            .unwrap()
            .split_once(" ")
            .unwrap();
        instructions
            .split(",")
            .map(|instruction| instruction.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
    };
    let computer = Computer {
        register_a,
        register_b,
        register_c,
        instruction_pointer: 0,
        program,
    };
    puzzle_1(computer.clone());
    puzzle_2(computer.clone());
}
