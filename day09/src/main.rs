// Update on day05
//
// Operations:
//
// - Halt has no params
// - In / Out have one param, the addr to write to / read from
// - Add / Mul have 3 params, lhs rhs output_addr
// - JumpIfTrue / JumpIfFalse have 2 params, cmp dest, (set self.ip = des, if cmp == true / false)
// - Less / Equals, lhs rhs output_addr, (output = lhs cmp rhs)
// - SetRelBase, val, sets the value of the relative base
//
// Ocpode now has up to 5 digits compared to day02:
//
// - First two digits are the actual op code
// - Last three digits are the param mode from right to left (right most is first param mode)
// - 0 means param specifies mem addr
// - 1 means param is a direct value
// - 2 means param specifies rel mem addr from rel base
//
// Omitted digits are 0 (output addr will always be 0 and probably be omitted)
//
// Some extra changes:
//
// - Program now has extra mem at the end
// - Program now suppoerts i64

// Notes:
//
// self.ip = instruction pointer
// mem = memory
// addr = address
// param = parameter
// val = value
// op = operation

const ONE_MB: usize = 1024 * 1024;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    In,
    Out,
    Halt,
    JumpIfTrue,
    JumpIfFalse,
    Less,
    Equals,
    SetRelBase,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ParamMode {
    Addr,
    Val,
    Rel,
}

fn to_digits(number: u32) -> Vec<u32> {
    fn inner(number: u32, result: &mut Vec<u32>) {
        if number >= 10 {
            inner(number / 10, result);
        }
        result.push(number % 10);
    }
    let mut result = Vec::new();
    inner(number, &mut result);
    result
}

fn to_op(val: u32, addr: usize) -> Op {
    match val {
        1 => Op::Add,
        2 => Op::Mul,
        3 => Op::In,
        4 => Op::Out,
        5 => Op::JumpIfTrue,
        6 => Op::JumpIfFalse,
        7 => Op::Less,
        8 => Op::Equals,
        9 => Op::SetRelBase,
        99 => Op::Halt,
        _ => panic!("Invalid op value {} at addr {}", val, addr),
    }
}

fn to_param_mode(val: u32, addr: usize) -> ParamMode {
    match val {
        0 => ParamMode::Addr,
        1 => ParamMode::Val,
        2 => ParamMode::Rel,
        _ => panic!("Invalid param mode value {} at addr {}", val, addr),
    }
}

// parse opcode and extract op and param modes
fn parse_opcode(opcode: u32, addr: usize) -> (Op, ParamMode, ParamMode, ParamMode) {
    let digits = to_digits(opcode);
    let num_digits = digits.len();
    let op_val = if opcode < 100 {
        opcode
    } else {
        10 * digits[num_digits - 2] + digits[num_digits - 1]
    };
    let op = to_op(op_val, addr);
    let param_mode_0 = if opcode < 100 {
        ParamMode::Addr
    } else {
        to_param_mode(digits[num_digits - 3], addr)
    };
    let param_mode_1 = if opcode < 1000 {
        ParamMode::Addr
    } else {
        to_param_mode(digits[num_digits - 4], addr)
    };
    let param_mode_2 = if opcode < 10000 {
        ParamMode::Addr
    } else {
        to_param_mode(digits[num_digits - 5], addr)
    };

    (op, param_mode_0, param_mode_1, param_mode_2)
}

fn get_user_input() -> i64 {
    println!("Enter input:");
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    input_text.trim().parse().unwrap()
}

struct Program {
    mem: Vec<i64>,
    ip: usize,
    rel_base: i64,
}

impl Program {
    fn new(mut mem: Vec<i64>) -> Self {
        // Add 100 MB of RAM to end
        mem.resize(mem.len() + 100 * ONE_MB, 0);
        Self {
            mem,
            ip: 0,
            rel_base: 0,
        }
    }

    fn get_param_val(&self, addr: usize, param_mode: ParamMode) -> i64 {
        match param_mode {
            ParamMode::Addr => self.mem[self.mem[addr] as usize],
            ParamMode::Val => self.mem[addr],
            ParamMode::Rel => self.mem[(self.rel_base + self.mem[addr]) as usize],
        }
    }

    fn get_result_addr(&self, addr: usize, param_mode: ParamMode) -> usize {
        match param_mode {
            ParamMode::Addr => self.mem[addr] as usize,
            ParamMode::Rel => (self.rel_base + self.mem[addr]) as usize,
            ParamMode::Val => unreachable!(),
        }
    }

    fn run(&mut self) {
        loop {
            let (op, param_mode_0, param_mode_1, param_mode_2) =
                parse_opcode(self.mem[self.ip] as u32, self.ip);
            match op {
                Op::Add => {
                    let lhs = self.get_param_val(self.ip + 1, param_mode_0);
                    let rhs = self.get_param_val(self.ip + 2, param_mode_1);
                    let result_addr = self.get_result_addr(self.ip + 3, param_mode_2);
                    self.mem[result_addr] = lhs + rhs;
                    self.ip += 4;
                }
                Op::Mul => {
                    let lhs = self.get_param_val(self.ip + 1, param_mode_0);
                    let rhs = self.get_param_val(self.ip + 2, param_mode_1);
                    let result_addr = self.get_result_addr(self.ip + 3, param_mode_2);
                    self.mem[result_addr] = lhs * rhs;
                    self.ip += 4;
                }
                Op::In => {
                    let user_input = get_user_input();
                    let result_addr = self.get_result_addr(self.ip + 1, param_mode_0);
                    self.mem[result_addr] = user_input;
                    self.ip += 2;
                }
                Op::Out => {
                    let val = self.get_param_val(self.ip + 1, param_mode_0);
                    println!("{}", val);
                    self.ip += 2;
                }
                Op::JumpIfTrue => {
                    let val = self.get_param_val(self.ip + 1, param_mode_0);
                    let new_ip = self.get_param_val(self.ip + 2, param_mode_1) as usize;
                    if val != 0 {
                        self.ip = new_ip;
                    } else {
                        self.ip += 3;
                    }
                }
                Op::JumpIfFalse => {
                    let val = self.get_param_val(self.ip + 1, param_mode_0);
                    let new_ip = self.get_param_val(self.ip + 2, param_mode_1) as usize;
                    if val == 0 {
                        self.ip = new_ip;
                    } else {
                        self.ip += 3;
                    }
                }
                Op::Less => {
                    let lhs = self.get_param_val(self.ip + 1, param_mode_0);
                    let rhs = self.get_param_val(self.ip + 2, param_mode_1);
                    let result_addr = self.get_result_addr(self.ip + 3, param_mode_2);
                    self.mem[result_addr] = if lhs < rhs { 1 } else { 0 };
                    self.ip += 4;
                }
                Op::Equals => {
                    let lhs = self.get_param_val(self.ip + 1, param_mode_0);
                    let rhs = self.get_param_val(self.ip + 2, param_mode_1);
                    let result_addr = self.get_result_addr(self.ip + 3, param_mode_2);
                    self.mem[result_addr] = if lhs == rhs { 1 } else { 0 };
                    self.ip += 4;
                }
                Op::SetRelBase => {
                    let val = self.get_param_val(self.ip + 1, param_mode_0);
                    self.rel_base += val;
                    self.ip += 2;
                }
                Op::Halt => break,
            }
        }
    }
}

fn part1(input: &[i64]) {
    // part1 expects input = 1
    let mem = input.to_vec();
    println!("start part1");
    let mut program = Program::new(mem);
    program.run();
    println!("exit part1");
}

fn part2(input: &[i64]) {
    // part2 expects input = 1
    let mem = input.to_vec();
    println!("start part2");
    let mut program = Program::new(mem);
    program.run();
    println!("exit part2");
}

fn main() {
    let input_str = include_str!("input.txt");
    let input = input_str
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<i64>, _>>()
        .unwrap();
    part1(&input);
    part2(&input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_opcode() {
        assert_eq!(
            parse_opcode(1, 0),
            (Op::Add, ParamMode::Addr, ParamMode::Addr, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(2, 0),
            (Op::Mul, ParamMode::Addr, ParamMode::Addr, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(3, 0),
            (Op::In, ParamMode::Addr, ParamMode::Addr, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(4, 0),
            (Op::Out, ParamMode::Addr, ParamMode::Addr, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(99, 0),
            (Op::Halt, ParamMode::Addr, ParamMode::Addr, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(101, 0),
            (Op::Add, ParamMode::Val, ParamMode::Addr, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(1001, 0),
            (Op::Add, ParamMode::Addr, ParamMode::Val, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(1101, 0),
            (Op::Add, ParamMode::Val, ParamMode::Val, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(201, 0),
            (Op::Add, ParamMode::Rel, ParamMode::Addr, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(2001, 0),
            (Op::Add, ParamMode::Addr, ParamMode::Rel, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(2101, 0),
            (Op::Add, ParamMode::Val, ParamMode::Rel, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(20101, 0),
            (Op::Add, ParamMode::Val, ParamMode::Addr, ParamMode::Rel)
        );
    }
}
