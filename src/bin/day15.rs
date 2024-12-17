use std::collections::{HashMap, HashSet, LinkedList, VecDeque};
use std::env::consts;
use std::fs::File;
use std::i32;
use std::io::{BufRead, BufReader};

fn main() {
    let (walls, mut boxes, moves, start) = read_data();
    println!("{}", walls.len());
    println!("{:?}", boxes);
    println!("{:?}", moves);
    println!("{:?}", start);

    let mut robot_position = start;
    for facing in moves {
        let move_to = next_free(&walls, &boxes, &robot_position, &facing);
        if let Some(free_position) = move_to {
            let adj_position = one_step(&robot_position, &facing);
            if boxes.contains(&adj_position) {
                boxes.remove(&adj_position);
                boxes.insert(free_position);
            }
            robot_position = adj_position;
        }
    }
    println!("{:?}", boxes);
    let sum = boxes
        .iter()
        .map(|position| position.x + 100 * position.y)
        .sum::<i32>();
    println!("{}", sum);
}

fn next_free(
    walls: &HashSet<Position>,
    boxes: &HashSet<Position>,
    position: &Position,
    facing: &Facing,
) -> Option<Position> {
    let new_position = one_step(position, facing);
    if walls.contains(&new_position) {
        None
    } else if boxes.contains(&new_position) {
        next_free(walls, boxes, &new_position, facing)
    } else {
        Some(new_position)
    }
}

fn one_step(position: &Position, facing: &Facing) -> Position {
    match facing {
        Facing::NORTH => Position {
            x: position.x,
            y: position.y - 1,
        },
        Facing::WEST => Position {
            x: position.x - 1,
            y: position.y,
        },
        Facing::SOUTH => Position {
            x: position.x,
            y: position.y + 1,
        },
        Facing::EAST => Position {
            x: position.x + 1,
            y: position.y,
        },
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
enum Facing {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

fn read_data() -> (HashSet<Position>, HashSet<Position>, Vec<Facing>, Position) {
    let mut walls: HashSet<Position> = HashSet::new();
    let mut boxes: HashSet<Position> = HashSet::new();
    let mut moves: Vec<Facing> = Vec::new();
    let mut start: Option<Position> = None;

    let file = File::open("data/day15.txt").expect("File not found");
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|l| l.expect("Couldnt read the file?"))
        .collect::<Vec<String>>();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                walls.insert(Position {
                    x: x as i32,
                    y: y as i32,
                });
            } else if c == 'O' {
                boxes.insert(Position {
                    x: x as i32,
                    y: y as i32,
                });
            } else if c == '@' {
                start = Some(Position {
                    x: x as i32,
                    y: y as i32,
                });
            } else if "v>^<".contains(c) {
                if c == '^' {
                    moves.push(Facing::NORTH);
                } else if c == 'v' {
                    moves.push(Facing::SOUTH);
                } else if c == '>' {
                    moves.push(Facing::EAST);
                } else if c == '<' {
                    moves.push(Facing::WEST);
                }
            }
        }
    }

    (walls, boxes, moves, start.unwrap())
}
