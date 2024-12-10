use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    prob2();
}
fn prob2() {
    let file = File::open("data/day6.txt").expect("File not found");

    let reader = BufReader::new(file);
    let map: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Couldnt read the file?"))
        .collect();

    //println!("{:?}", map);

    // let y_size = map.len();
    // let x_size = map[0].len();

    let mut position = get_initial_position(&map);
    //let mut pos_opt = Some(position);

    println!("Initial Position {:?}", position);

    let mut covered_positions: HashMap<(i32, i32), i32> = HashMap::new();

    //covered_positions.insert((position.x, position.y));

    while move_next_position(&mut position, &map) {
        //println!("position {:?} covered {:?}", position,covered_positions);
        covered_positions.insert((position.x, position.y), 0);
    }
    println!("result {}", covered_positions.len());
}

fn prob1() {
    let file = File::open("data/day6.txt").expect("File not found");

    let reader = BufReader::new(file);
    let map: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Couldnt read the file?"))
        .collect();

    //println!("{:?}", map);

    // let y_size = map.len();
    // let x_size = map[0].len();

    let mut position = get_initial_position(&map);
    //let mut pos_opt = Some(position);

    println!("Initial Position {:?}", position);

    let mut covered_positions: HashSet<(i32, i32)> = HashSet::new();
    covered_positions.insert((position.x, position.y));

    while move_next_position(&mut position, &map) {
        //println!("position {:?} covered {:?}", position,covered_positions);
        covered_positions.insert((position.x, position.y));
    }
    println!("result {}", covered_positions.len());
}

fn move_next_position(position: &mut Position, map: &Vec<String>) -> bool {
    let (next_x, next_y) = position.next_step();
    let next_position = if is_out(next_x, next_y, map) {
        false
    } else {
        let next = get(next_x, next_y, map);
        //println!("next {} {} {}" ,next, next_x, next_y);
        match next {
            '.' | '^' | 'v' | '>' | '<' => {
                position.move_to(next_x, next_y);
                true
            }
            '#' => {
                position.turn();
                move_next_position(position, map)
            }
            _ => panic!("Shouldnt happen {:?}", position),
        }
    };
    next_position
}
fn get(x: i32, y: i32, map: &Vec<String>) -> char {
    map[usize::try_from(y).unwrap()]
        .chars()
        .nth(usize::try_from(x).unwrap())
        .unwrap()
}

fn is_out(x: i32, y: i32, map: &Vec<String>) -> bool {
    x < 0 || y < 0 || x >= map[0].len().try_into().unwrap() || y >= map.len().try_into().unwrap()
}

fn get_initial_position(map: &Vec<String>) -> Position {
    let result = map.iter().enumerate().find_map(|(y, row)| {
        row.chars()
            .enumerate()
            .find(|(x, c)| "<>^v".contains(*c))
            .map(|(x, c)| Position::create(&c, x, y))
    });
    result.unwrap()
}
#[derive(Debug)]
enum Facing {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    facing: Facing,
}

impl Position {
    fn turn(&mut self) {
        self.facing = match self.facing {
            Facing::UP => Facing::RIGHT,
            Facing::RIGHT => Facing::DOWN,
            Facing::DOWN => Facing::LEFT,
            Facing::LEFT => Facing::UP,
        }
    }

    fn next_step(&self) -> (i32, i32) {
        match self.facing {
            Facing::UP => (self.x, self.y - 1),
            Facing::RIGHT => (self.x + 1, self.y),
            Facing::DOWN => (self.x, self.y + 1),
            Facing::LEFT => (self.x - 1, self.y),
        }
    }
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn create(c: &char, x: usize, y: usize) -> Self {
        let facing = match c {
            '>' => Facing::RIGHT,
            '<' => Facing::LEFT,
            '^' => Facing::UP,
            'v' => Facing::DOWN,
            _ => panic!("not facing anywhere {c}"),
        };
        Self {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
            facing,
        }
    }
}
