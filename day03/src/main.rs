fn part1(lines: &[LineString]) {
    let line_string0 = &lines[0];
    let line_string1 = &lines[1];
    let mut p_min = Point::new(99_999_999, 99_999_999);
    for s0 in line_string0.segments().iter() {
        for s1 in line_string1.segments().iter() {
            if s0.a == s1.a && s0.a == Point::new(0, 0) {
                continue;
            }
            if let Some(point) = intersection(*s0, *s1) {
                if point.manhattan_distance() < p_min.manhattan_distance() {
                    p_min = point;
                }
            }
        }
    }
    println!("Part1: {}", p_min.manhattan_distance());
}

fn part2(lines: &[LineString]) {
    let line_string0 = &lines[0];
    let line_string1 = &lines[1];
    let mut steps_s0 = 0;
    let mut min_steps_sum = 999_999_999;
    for s0 in line_string0.segments().iter() {
        let mut steps_s1 = 0;
        for s1 in line_string1.segments().iter() {
            if s0.a == s1.a && s0.a == Point::new(0, 0) {
                continue;
            }

            if let Some(point) = intersection(*s0, *s1) {
                let steps_s0_isec = steps_s0 + Segment::new(s0.a, point).manhattan_distance();
                let steps_s1_isec = steps_s1 + Segment::new(s1.a, point).manhattan_distance();
                let steps_sum = steps_s0_isec + steps_s1_isec;
                if steps_sum < min_steps_sum {
                    min_steps_sum = steps_sum;
                }
            }
            steps_s1 += s1.manhattan_distance();
        }
        steps_s0 += s0.manhattan_distance();
    }
    println!("Part2: {}", min_steps_sum);
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone)]
struct Move {
    direction: Direction,
    steps: i32,
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        // Input is U|R|D|L<integer> e.g. "R5123" for Right 5123 steps
        let first = input.chars().next().unwrap();
        let direction = match first {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => panic!("Invalid Direction: {}", first),
        };
        let steps = input[1..].parse().unwrap();
        Move { direction, steps }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn manhattan_distance(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Copy, Clone)]
struct Segment {
    a: Point,
    b: Point,
}

impl Segment {
    pub fn new(a: Point, b: Point) -> Self {
        Segment { a, b }
    }

    pub fn manhattan_distance(self) -> i32 {
        (self.a.x - self.b.x).abs() + (self.a.y - self.b.y).abs()
    }
}

fn intersection(s0: Segment, s1: Segment) -> Option<Point> {
    let s0_is_vertical = s0.a.x == s0.b.x;
    let s0_is_horizontal = s0.a.y == s0.b.y;
    let s1_is_vertical = s1.a.x == s1.b.x;
    let s1_is_horizontal = s1.a.y == s1.b.y;

    assert_ne!(s0_is_vertical, s0_is_horizontal);
    assert_ne!(s1_is_vertical, s1_is_horizontal);

    // parallel segments (we assume they never overlap each other while being parallel)
    if (s0_is_horizontal && s1_is_horizontal) || (s0_is_vertical && s1_is_vertical) {
        return None;
    }

    // not intersecting
    //
    // s0 is completely to the right / left / top / bottom of s1
    if (s0.a.x > s1.a.x && s0.a.x > s1.b.x && s0.b.x > s1.a.x && s0.b.x > s1.b.x)
        || (s0.a.x < s1.a.x && s0.a.x < s1.b.x && s0.b.x < s1.a.x && s0.b.x < s1.b.x)
        || (s0.a.y > s1.a.y && s0.a.y > s1.b.y && s0.b.y > s1.a.y && s0.b.y > s1.b.y)
        || (s0.a.y < s1.a.y && s0.a.y < s1.b.y && s0.b.y < s1.a.y && s0.b.y < s1.b.y)
    {
        return None;
    }

    let point = if s0_is_vertical {
        assert!(s1_is_horizontal);
        Point::new(s0.a.x, s1.a.y)
    } else {
        assert!(s0_is_horizontal);
        assert!(s1_is_vertical);
        Point::new(s1.a.x, s0.a.y)
    };
    Some(point)
}

#[derive(Debug, Clone)]
struct LineString {
    points: Vec<Point>,
}

impl LineString {
    pub fn new(moves: &[Move]) -> Self {
        assert!(!moves.is_empty());
        let mut points = Vec::with_capacity(moves.len() + 1);
        let mut point = Point::new(0, 0);
        points.push(point);
        for m in moves {
            match m.direction {
                Direction::Left => point.x -= m.steps,
                Direction::Right => point.x += m.steps,
                Direction::Up => point.y += m.steps,
                Direction::Down => point.y -= m.steps,
            }
            points.push(point);
        }
        LineString { points }
    }

    pub fn segments(&self) -> Vec<Segment> {
        self.points
            .windows(2)
            .map(|p| Segment::new(p[0], p[1]))
            .collect()
    }
}

impl From<&str> for LineString {
    fn from(input: &str) -> Self {
        let moves: Vec<Move> = input.split(',').map(Into::into).collect();
        LineString::new(&moves)
    }
}

fn main() {
    let input_str = include_str!("input.txt");
    let input: Vec<LineString> = input_str
        .trim()
        .split_whitespace()
        .map(Into::into)
        .collect();
    assert_eq!(input.len(), 2);
    part1(&input);
    part2(&input);
}
