use std::fmt::{self, Debug};
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    //prob1();
    prob2();
}
fn prob2() {
    let (_, b, c, program) = read_data();
    let mut a = 0;

    for i in (0..program.len()).rev() {
        let value = program[i];

        let to_match = program
            .iter()
            .enumerate()
            .filter(|(index, _)| *index >= i)
            .map(|(_, v)| *v)
            .collect::<Vec<u64>>();
        println!("to_match= {:?}", to_match);
        loop {
            a = find_next_a(a, 0, 0, value);
            //
            let mut computer = Computer { a, b, c };
            let result = computer.run_program(&program);

            if result == to_match {
                println!("result  = {:?}", result);
                println!("a ={a}");
                break;
            }
        }
        a = a * 8;
    }
}
fn prob23() {
    let (_, b, c, program) = read_data();
    let mut a = 0;

    let mut counter = 0;
    loop {
        a = find_next_a(a, b, c, program[0]);

        let mut computer = Computer { a, b, c };
        let response = computer.run_program(&program);
        //println!("program {:?}", program);
        if response.len() > 0 {
            println!("{a}");
            println!("response {:?}", response);
        }
        counter += 1;
        if response == program {
            break;
        }
    }
    println!("{a}")
}
fn find_next_a(start_a: u64, b: u64, c: u64, value: u64) -> u64 {
    let mut counter = 0;
    loop {
        counter = counter + 1;
        let a = start_a + counter;
        let (_, result, _) = compute_one(a, b, c, 1);
        if result % 8 == value {
            return a;
        }
    }
}
fn compute_one(mut a: u64, mut b: u64, mut c: u64, count: i32) -> (u64, u64, u64) {
    for _ in 0..count {
        b = a % 8;
        b = b ^ 2;
        c = a / 2_u64.pow(b as u32);
        b = b ^ c;
        a = a / 8;
        b = b ^ 7;
    }
    (a, b, c)
}
fn prob22() {
    let (_, b, c, program) = read_data();
    let mut a = 0;
    println!("{} {} {} {:?}", a, b, c, program);

    loop {
        if a % 100000 == 0 {
            println!("{a}");
        }
        let mut computer = Computer { a, b, c };
        let response = computer.run_program(&program);
        if response == program {
            break;
        }
        a = a + 1;
    }
    println!("{a}");
}

fn prob1() {
    let (a, b, c, program) = read_data();
    println!("{} {} {} {:?}", a, b, c, program);
    let mut computer = Computer { a, b, c };
    let response = computer.run_program(&program);
    println!("{:?}", response)
}

fn read_data() -> (u64, u64, u64, Vec<u64>) {
    let file = File::open("data/day17.txt").expect("File not found");
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|l| l.expect("Couldnt read the file?"))
        .collect::<Vec<String>>();

    let mut a: Option<u64> = None;
    let mut b: Option<u64> = None;
    let mut c: Option<u64> = None;
    let mut program: Vec<u64> = Vec::new();

    for line in lines {
        if line.starts_with("Register A") {
            a = Some(get_register(&line));
        } else if line.starts_with("Register B") {
            b = Some(get_register(&line));
        } else if line.starts_with("Register C") {
            c = Some(get_register(&line));
        } else if line.starts_with("Program:") {
            let (_, program_string) = line.split_at(9);
            //println!("{}", program_string);
            program = program_string
                .split(",")
                .map(|c| c.parse::<u64>())
                .map(|s| s.unwrap() as u64)
                .collect::<Vec<u64>>();
        }
    }
    //println!("{:?} {:?} {:?} {:?}", a, b, c, program);
    (a.unwrap(), b.unwrap(), c.unwrap(), program)
}

fn get_register(line: &str) -> u64 {
    let (_, value) = line.split_at(12);
    value.parse().unwrap()
}

struct Computer {
    a: u64,
    b: u64,
    c: u64,
}

impl Debug for Computer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C[a={} b={} c={}]", self.a, self.b, self.c)
    }
}
impl Computer {
    fn run_program(&mut self, program: &Vec<u64>) -> Vec<u64> {
        let mut pointer: usize = 0;
        let mut final_output: Vec<u64> = Vec::new();

        while pointer < program.len() {
            let instruction = program[pointer];
            let operand = program[pointer + 1];
            //println!("{} {} {} {:?}", pointer, instruction, operand, computer);
            let response = self.execute(instruction, operand, pointer);
            pointer = response.0;
            if let Some(output) = response.1 {
                final_output.push(output);

                // for i in 0..final_output.len() {
                //     if final_output[i] != program[i] {
                //         return Vec::new();
                //     }
                // }
            }
        }
        final_output
    }
    fn execute(&mut self, instruction: u64, operand: u64, pointer: usize) -> (usize, Option<u64>) {
        let mut next_pointer = pointer + 2;
        let mut out_value = None;
        if instruction == 0 {
            //adv
            self.a = self.a / 2_u64.pow(self.get_combo(operand) as u32);
        } else if instruction == 1 {
            //bxl
            self.b = self.b ^ operand;
        } else if instruction == 2 {
            //bst
            self.b = self.get_combo(operand) % 8;
        } else if instruction == 3 {
            //jnz
            if self.a != 0 {
                next_pointer = usize::try_from(operand).unwrap();
            }
        } else if instruction == 4 {
            //bxc
            self.b = self.b ^ self.c;
        } else if instruction == 5 {
            //out
            out_value = Some(self.get_combo(operand) % 8);
        } else if instruction == 6 {
            //bdv
            self.b = self.a / 2_u64.pow(self.get_combo(operand) as u32);
        } else if instruction == 7 {
            //cdv
            self.c = self.a / 2_u64.pow(self.get_combo(operand) as u32);
        }
        (next_pointer, out_value)
    }

    fn get_combo(&self, operand: u64) -> u64 {
        if operand <= 3 {
            operand
        } else if operand == 4 {
            self.a
        } else if operand == 5 {
            self.b
        } else if operand == 6 {
            self.c
        } else {
            panic!("incorrect combo {}", operand);
        }
    }
}
