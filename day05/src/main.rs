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

// Update on day02
//
// Halt has no parameters
// In / Out have one parameter, the address to write to / read from
// Add / Mul have 3 parameters, lhs rhs output_address
// JumpIfTrue / JumpIfFalse have 2 parameters, cmp dest, (set ip = des, if cmp == true / false)
// Less / Equals, lhs rhs output_address, (output = lhs cmp rhs)
//
//
// Ocpode now has up to 5 digits compared to day02
//
// First two digits are the actual op code
// Last three digits are the parameter mode from right to left (right most is first parameter mode)
// 0 means parameter specifies mem address
// 1 means parameter is a direct value
//
// Omitted digits are 0 (output address will always be 0 and probably be omitted)

// Notes:
//
// ip = instruction pointer
// mem = memory
// addr = address
// param = parameter
// val = value
// op = operation

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

fn get_param_val(mem: &mut [i32], addr: usize, param_mode: ParamMode) -> i32 {
    match param_mode {
        ParamMode::Addr => mem[mem[addr] as usize],
        ParamMode::Val => mem[addr],
    }
}

fn get_user_input() -> i32 {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).unwrap();
    input_text.trim().parse().unwrap()
}

fn run_program(mem: &mut [i32]) {
    let mut ip = 0;
    loop {
        let (op, param_mode_0, param_mode_1) = parse_opcode(mem[ip] as u32, ip);
        match op {
            Op::Add => {
                let lhs = get_param_val(mem, ip + 1, param_mode_0);
                let rhs = get_param_val(mem, ip + 2, param_mode_1);
                let result_addr = mem[ip + 3] as usize;
                mem[result_addr] = lhs + rhs;
                ip += 4;
            }
            Op::Mul => {
                let lhs = get_param_val(mem, ip + 1, param_mode_0);
                let rhs = get_param_val(mem, ip + 2, param_mode_1);
                let result_addr = mem[ip + 3] as usize;
                mem[result_addr] = lhs * rhs;
                ip += 4;
            }
            Op::In => {
                let user_input = get_user_input();
                let result_addr = mem[ip + 1] as usize;
                mem[result_addr] = user_input;
                ip += 2;
            }
            Op::Out => {
                let val = get_param_val(mem, ip + 1, param_mode_0);
                println!("{}", val);
                ip += 2;
            }
            Op::JumpIfTrue => {
                let val = get_param_val(mem, ip + 1, param_mode_0);
                let new_ip = get_param_val(mem, ip + 2, param_mode_1) as usize;
                if val != 0 {
                    ip = new_ip;
                } else {
                    ip += 3;
                }
            }
            Op::JumpIfFalse => {
                let val = get_param_val(mem, ip + 1, param_mode_0);
                let new_ip = get_param_val(mem, ip + 2, param_mode_1) as usize;
                if val == 0 {
                    ip = new_ip;
                } else {
                    ip += 3;
                }
            }
            Op::Less => {
                let lhs = get_param_val(mem, ip + 1, param_mode_0);
                let rhs = get_param_val(mem, ip + 2, param_mode_1);
                let result_addr = mem[ip + 3] as usize;
                mem[result_addr] = if lhs < rhs { 1 } else { 0 };
                ip += 4;
            }
            Op::Equals => {
                let lhs = get_param_val(mem, ip + 1, param_mode_0);
                let rhs = get_param_val(mem, ip + 2, param_mode_1);
                let result_addr = mem[ip + 3] as usize;
                mem[result_addr] = if lhs == rhs { 1 } else { 0 };
                ip += 4;
            }
            Op::Halt => break,
        }
    }
}

fn part1(input: &[i32]) {
    let mut mem = input.to_vec();
    println!("start part1");
    run_program(&mut mem);
    println!("exit part1");
}

fn part2(input: &[i32]) {
    let mut mem = input.to_vec();
    println!("start part2");
    run_program(&mut mem);
    println!("exit part2");
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_opcode() {
        assert_eq!(
            parse_opcode(1, 0),
            (Op::Add, ParamMode::Addr, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(2, 0),
            (Op::Mul, ParamMode::Addr, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(3, 0),
            (Op::In, ParamMode::Addr, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(4, 0),
            (Op::Out, ParamMode::Addr, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(99, 0),
            (Op::Halt, ParamMode::Addr, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(101, 0),
            (Op::Add, ParamMode::Val, ParamMode::Addr)
        );
        assert_eq!(
            parse_opcode(1001, 0),
            (Op::Add, ParamMode::Addr, ParamMode::Val)
        );
        assert_eq!(
            parse_opcode(1101, 0),
            (Op::Add, ParamMode::Val, ParamMode::Val)
        );
    }
}
