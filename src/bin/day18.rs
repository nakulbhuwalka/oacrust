use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

const GRID_SIZE: i32 = 70 + 1;
const COUNT: usize = 1024;

fn main() {
    let mut trail = HashMap::new();
    trail.insert(Position { x: 5, y: 6 }, 5);

    let val = get_next_positions(&HashSet::new(), 10, &Position { x: 5, y: 5 }, &trail, 4);

    // println!("{:?}", val);
    prob2();
}

fn prob2() {
    let incoming = read_data();

    let mut start = COUNT;
    let mut end = incoming.len();
    loop {
        let middle = (start + end) / 2;
        if start >= end || start == middle || end == middle {
            break;
        }
        let corrupted = get_corrupted(&incoming, middle);

        let min_steps = shortest_path(
            &Position { x: 0, y: 0 },
            &corrupted,
            GRID_SIZE,
            &mut HashMap::new(),
            0,
        );

        println!(
            "{} {} middle={} {:?} {:?}",
            start,
            end,
            middle,
            incoming[middle - 1],
            min_steps
        );
        if min_steps.is_none() {
            end = middle;
        } else {
            start = middle;
        }
    }

    // let corrupted = get_corrupted(&incoming, COUNT);
    // println!("{:?}", corrupted);
}

fn prob1() {
    let incoming = read_data();

    let corrupted = get_corrupted(&incoming, COUNT);
    // println!("{:?}", corrupted);

    let min_steps = shortest_path(
        &Position { x: 0, y: 0 },
        &corrupted,
        GRID_SIZE,
        &mut HashMap::new(),
        0,
    );

    println!("{:?}", min_steps)
}
fn shortest_path(
    position: &Position,
    corrupted: &HashSet<Position>,
    grid_size: i32,
    trail: &mut HashMap<Position, i32>,
    steps_so_far: i32,
) -> Option<i32> {
    trail.insert(*position, steps_so_far);
    let next_positions =
        get_next_positions(corrupted, grid_size, position, trail, steps_so_far + 1);

    //println!("position {:?} steps {}", position, steps_so_far);
    //println!("next_positions {:?}", next_positions);
    let mut min_steps = None;

    for next_position in next_positions.iter() {
        if next_position.x == grid_size - 1 && next_position.y == grid_size - 1 {
            min_steps = Some(steps_so_far + 1);
            //println!("found {:?}", trail);
            //   println!("steps {:?}", min_steps);
            break;
        } else {
            let steps = shortest_path(next_position, corrupted, grid_size, trail, steps_so_far + 1);
            if min_steps.is_none() {
                min_steps = steps;
            } else if steps.is_some() {
                if steps.unwrap() < min_steps.unwrap() {
                    min_steps = steps
                }
            }
        }
    }
    // if min_steps.is_none() {
    //     println!("dead end {:?}", position)
    // }

    min_steps
}

fn read_data() -> Vec<Position> {
    let file = File::open("data/day18.txt").expect("File not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Couldnt read the file?"))
        .map(|line| {
            let numbers = line
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            Position {
                x: numbers[0],
                y: numbers[1],
            }
        })
        .collect::<Vec<Position>>()
}

fn get_corrupted(incoming: &Vec<Position>, count: usize) -> HashSet<Position> {
    let mut corrupted = HashSet::new();
    corrupted.extend(incoming[0..count].iter());
    corrupted
}

fn get_next_positions(
    corrupted: &HashSet<Position>,
    grid_size: i32,
    current_position: &Position,
    trail: &HashMap<Position, i32>,
    steps_so_far: i32,
) -> Vec<Position> {
    current_position
        .get_adj()
        .into_iter()
        //.inspect(|p| println!("next {:?}", p))
        .filter(|position| position.is_valid(grid_size, corrupted))
        .filter(|position| {
            trail.get(position).is_none()
                || trail
                    .get(position)
                    .filter(|previous_steps| steps_so_far < **previous_steps)
                    .is_some()
        })
        .collect::<Vec<Position>>()
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn north(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn south(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn get_adj(&self) -> [Position; 4] {
        [self.south(), self.east(), self.north(), self.west()]
    }
    fn is_valid(&self, grid_size: i32, corrupted: &HashSet<Position>) -> bool {
        self.x >= 0
            && self.y >= 0
            && self.x < grid_size
            && self.y < grid_size
            && !corrupted.contains(&self)
    }
}
