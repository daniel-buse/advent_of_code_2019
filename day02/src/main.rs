const OP_ADD: i32 = 1;
const OP_MUL: i32 = 2;
const OP_HALT: i32 = 99;

const PART2_PROGRAM_OUTPUT: i32 = 19_690_720;

fn run_program(memory: &mut [i32]) {
    let mut i = 0;
    loop {
        let opcode = memory[i];
        match opcode {
            OP_ADD => {
                let lhs_pos = memory[i + 1] as usize;
                let rhs_pos = memory[i + 2] as usize;
                let result_pos = memory[i + 3] as usize;
                memory[result_pos] = memory[lhs_pos] + memory[rhs_pos];
            }
            OP_MUL => {
                let lhs_pos = memory[i + 1] as usize;
                let rhs_pos = memory[i + 2] as usize;
                let result_pos = memory[i + 3] as usize;
                memory[result_pos] = memory[lhs_pos] * memory[rhs_pos];
            }
            OP_HALT => break,
            _ => panic!("Unexpected opcode {}", opcode),
        }
        i += 4;
    }
}

fn part1(input: &[i32]) {
    let mut memory = input.to_vec();
    memory[1] = 12;
    memory[2] = 2;
    run_program(&mut memory);
    println!("Part1: {}", memory[0]);
}

fn part2(input: &[i32]) {
    for verb in 0..100 {
        for noun in 0..100 {
            let mut memory = input.to_vec();
            memory[1] = noun;
            memory[2] = verb;
            run_program(&mut memory);
            if memory[0] == PART2_PROGRAM_OUTPUT {
                println!("Part2: {}", 100 * noun + verb);
                return;
            }
        }
    }
    panic!("Did not find requested program output");
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
