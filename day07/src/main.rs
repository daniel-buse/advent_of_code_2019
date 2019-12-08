// Update on day05
//
// - Store mem and ip in struct, since we now need to keep state over multiple runs
// - Init program with a phase input
// - Run returns Some(i32) on output and None on halt

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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ParamMode {
    Addr,
    Val,
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
        99 => Op::Halt,
        _ => panic!("Invalid op value {} at addr {}", val, addr),
    }
}

fn to_param_mode(val: u32, addr: usize) -> ParamMode {
    match val {
        0 => ParamMode::Addr,
        1 => ParamMode::Val,
        _ => panic!("Invalid param mode value {} at addr {}", val, addr),
    }
}

// parse opcode and extract op and param modes
fn parse_opcode(opcode: u32, addr: usize) -> (Op, ParamMode, ParamMode) {
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

    (op, param_mode_0, param_mode_1)
}

struct Program {
    mem: Vec<i32>,
    ip: usize,
}

impl Program {
    pub fn new(mut mem: Vec<i32>, phase: i32) -> Self {
        let (op, _, _) = parse_opcode(mem[0] as u32, 0);
        // set phase in mem and advance ip to addr 2
        match op {
            Op::In => {
                let result_addr = mem[1] as usize;
                mem[result_addr] = phase;
            }
            _ => panic!("Expected OP::In at addr 0, got {:?}", op),
        }
        Self { mem, ip: 2 }
    }

    pub fn run(&mut self, input: i32) -> Option<i32> {
        loop {
            let (op, param_mode_0, param_mode_1) = parse_opcode(self.mem[self.ip] as u32, self.ip);
            match op {
                Op::Add => {
                    let lhs = self.get_param_val(self.ip + 1, param_mode_0);
                    let rhs = self.get_param_val(self.ip + 2, param_mode_1);
                    let result_addr = self.mem[self.ip + 3] as usize;
                    self.mem[result_addr] = lhs + rhs;
                    self.ip += 4;
                }
                Op::Mul => {
                    let lhs = self.get_param_val(self.ip + 1, param_mode_0);
                    let rhs = self.get_param_val(self.ip + 2, param_mode_1);
                    let result_addr = self.mem[self.ip + 3] as usize;
                    self.mem[result_addr] = lhs * rhs;
                    self.ip += 4;
                }
                Op::In => {
                    let result_addr = self.mem[self.ip + 1] as usize;
                    self.mem[result_addr] = input;
                    self.ip += 2;
                }
                Op::Out => {
                    let val = self.get_param_val(self.ip + 1, param_mode_0);
                    self.ip += 2;
                    return Some(val);
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
                    let result_addr = self.mem[self.ip + 3] as usize;
                    self.mem[result_addr] = if lhs < rhs { 1 } else { 0 };
                    self.ip += 4;
                }
                Op::Equals => {
                    let lhs = self.get_param_val(self.ip + 1, param_mode_0);
                    let rhs = self.get_param_val(self.ip + 2, param_mode_1);
                    let result_addr = self.mem[self.ip + 3] as usize;
                    self.mem[result_addr] = if lhs == rhs { 1 } else { 0 };
                    self.ip += 4;
                }
                Op::Halt => return None,
            }
        }
    }

    pub fn get_param_val(&mut self, addr: usize, param_mode: ParamMode) -> i32 {
        match param_mode {
            ParamMode::Addr => self.mem[self.mem[addr] as usize],
            ParamMode::Val => self.mem[addr],
        }
    }
}

fn next_permutation<T: std::cmp::Ord>(array: &mut [T]) -> bool {
    /*
     * Next lexicographical permutation algorithm (Rust)
     * by Project Nayuki, 2017. Public domain.
     * https://www.nayuki.io/page/next-lexicographical-permutation-algorithm
     */
    // Find non-increasing suffix
    if array.is_empty() {
        return false;
    }
    let mut i: usize = array.len() - 1;
    while i > 0 && array[i - 1] >= array[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }

    // Find successor to pivot
    let mut j: usize = array.len() - 1;
    while array[j] <= array[i - 1] {
        j -= 1;
    }
    array.swap(i - 1, j);

    // Reverse suffix
    array[i..].reverse();
    true
}

fn part1(input: &[i32]) {
    let mut max = 0;
    let mut phases = vec![0, 1, 2, 3, 4];
    loop {
        let mut programs = Vec::with_capacity(5);
        for phase in &phases {
            programs.push(Program::new(input.to_vec(), *phase));
        }
        let mut in_out = 0;
        for program in &mut programs {
            in_out = program.run(in_out).unwrap();
        }
        max = max.max(in_out);
        if !next_permutation(&mut phases) {
            break;
        }
    }

    println!("Part1: {}", max);
}

fn part2(input: &[i32]) {
    let mut max = 0;
    let mut phases = vec![5, 6, 7, 8, 9];
    loop {
        let mut programs = Vec::with_capacity(5);
        for phase in &phases {
            programs.push(Program::new(input.to_vec(), *phase));
        }
        let mut in_out = 0;
        let mut done = false;
        while !done {
            for program in &mut programs {
                match program.run(in_out) {
                    Some(val) => in_out = val,
                    None => done = true,
                }
            }
        }
        max = max.max(in_out);
        if !next_permutation(&mut phases) {
            break;
        }
    }

    println!("Part2: {}", max);
}

fn main() {
    let input_str = include_str!("input.txt");
    let input = input_str
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();
    part1(&input);
    part2(&input);
}
