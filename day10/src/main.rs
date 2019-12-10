const WIDTH: usize = 21;
const HEIGHT: usize = 21;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldType {
    Empty,
    Asteroid,
}

fn gcd(mut m: i32, mut n: i32) -> i32 {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n.abs()
}

fn gen_directions() -> Vec<(i32, i32)> {
    let mut directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    for y in 1..HEIGHT {
        for x in 1..WIDTH {
            let x = x as i32;
            let y = y as i32;
            let d = gcd(x, y);
            directions.push((x / d, y / d));
            directions.push((-x / d, y / d));
            directions.push((x / d, -y / d));
            directions.push((-x / d, -y / d));
        }
    }
    directions.sort();
    directions.dedup();
    directions
}

fn hits_asteroid(input: &[FieldType], start_pos: (usize, usize), dir: (i32, i32)) -> bool {
    assert!(!(dir.0 == 0 && dir.1 == 0));
    let mut x = start_pos.0 as i32;
    let mut y = start_pos.1 as i32;
    loop {
        x += dir.0;
        y += dir.1;
        if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 {
            return false;
        }
        let x = x as usize;
        let y = y as usize;
        let index = x + y * WIDTH;
        if input[index] == FieldType::Asteroid {
            return true;
        }
    }
}

fn part1(input: &[FieldType]) {
    let directions = gen_directions();
    let mut max = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index = x + y * WIDTH;
            if input[index] == FieldType::Empty {
                continue;
            }
            let mut curr_count = 0;
            for dir in &directions {
                if hits_asteroid(input, (x, y), *dir) {
                    curr_count += 1;
                }
            }
            max = max.max(curr_count);
        }
    }
    println!("Part1: {}", max);
}

fn part2(_input: &[FieldType]) {}

fn main() {
    let input_str = include_str!("input.txt");
    let input: Vec<FieldType> = input_str
        .trim()
        .split_whitespace()
        .map(|s| {
            s.chars().map(|c| {
                if c == '.' {
                    FieldType::Empty
                } else {
                    FieldType::Asteroid
                }
            })
        })
        .flatten()
        .collect();
    part1(&input);
    part2(&input);
}
