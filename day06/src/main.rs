const COM: &str = "COM";
const START: &str = "YOU";
const GOAL: &str = "SAN";

use std::collections::HashMap;

// get path between start and goal (not including start and goeal)
fn get_path<'a>(orbits: &HashMap<&'a str, &'a str>, start: &'a str, goal: &'a str) -> Vec<&'a str> {
    let mut path = Vec::new();
    let mut orbitee = orbits[start];
    while orbitee != goal {
        path.push(orbitee);
        orbitee = orbits[orbitee];
    }
    path
}

fn part1(orbits: &HashMap<&str, &str>) {
    let mut count = 0;
    for orbiter in orbits.keys() {
        let mut orbitee = orbits[orbiter];
        while orbitee != COM {
            orbitee = orbits[orbitee];
            count += 1;
        }
        count += 1;
    }
    println!("Part1: {}", count);
}

fn part2(orbits: &HashMap<&str, &str>) {
    let start_to_com_path = get_path(orbits, START, COM);
    let goal_to_com_path = get_path(orbits, GOAL, COM);
    // find the first common planet
    // the path lenght to that planet will be the planets index in the vec
    // just add both of them to get the travel distance between YOU to SAN
    for (planet_index, planet) in start_to_com_path.iter().enumerate() {
        for (planet2_index, planet2) in goal_to_com_path.iter().enumerate() {
            if planet == planet2 {
                println!("Part2: {}", planet_index + planet2_index);
                return;
            }
        }
    }
    panic!("Did not find common planet between YOU and SAN");
}

fn main() {
    // Input is lines of:
    // ABC)DEF
    //
    // where ABC and DEF are names of the planets
    // ABC)DEF means DEF orbits around ABC
    // COM is center of mass, bascially the end marker
    // (COM should only be on the left)
    //
    // orbiter = planet that orbits around something (DEF)
    // orbitee = planet that is orbited around (ABC)
    let input_str = include_str!("input.txt");
    let input: HashMap<&str, &str> = input_str
        .trim()
        .split_whitespace()
        .map(|s| {
            let mut split = s.split(')');
            let orbitee = split.next().unwrap();
            let orbiter = split.next().unwrap();
            (orbiter, orbitee)
        })
        .collect();
    part1(&input);
    part2(&input);
}
