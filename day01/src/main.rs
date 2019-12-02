fn required_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn part1(input: &[i32]) {
    let mut result = 0;
    for val in input {
        result += required_fuel(*val);
    }
    println!("Part1: {}", result);
}

fn part2(input: &[i32]) {
    let mut result = 0;
    for val in input {
        let mut mass = *val;
        loop {
            let fuel = required_fuel(mass);
            if fuel <= 0 {
                break;
            }
            result += fuel;
            mass = fuel;
        }
    }
    println!("Part2: {}", result);
}

fn main() {
    let input_str = include_str!("input.txt");
    let input = input_str
        .trim()
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();
    part1(&input);
    part2(&input);
}
