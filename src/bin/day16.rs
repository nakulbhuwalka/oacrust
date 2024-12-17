use std::collections::{HashMap, HashSet, LinkedList, VecDeque};
use std::env::consts;
use std::fs::File;
use std::i32;
use std::io::{BufRead, BufReader};

fn main() {
    let maze = read_data();
    prob1(&maze);
}

fn prob1(maze: &Maze) {
    let mut traversed: HashMap<Position, i32> = HashMap::new();

    let cost = traverse(
        &mut traversed,
        Facing::EAST,
        &maze.start,
        0,
        maze,
        &maze.start,
    );

    println!("{:?}", cost);

    let mut set = HashSet::new();
    set.extend(cost.unwrap().1);

    println!("{}", set.len())
}

fn traverse(
    traversed: &mut HashMap<Position, i32>,
    facing: Facing,
    position: &Position,
    cost: i32,
    maze: &Maze,
    last_position: &Position,
) -> Option<(i32, Vec<Position>)> {
    if *position == maze.end {
        return Some((cost, Vec::new()));
    }
    if let Some(traversed_cost) = traversed.get(position) {
        if *traversed_cost < cost {
            return None;
        }
    }
    traversed.insert(*position, cost);

    let forward_pos = maze.move_forward(position, &facing);
    let left_pos = maze.move_forward(position, &facing.do_turn(Turn::LEFT));
    let right_pos = maze.move_forward(position, &facing.do_turn(Turn::RIGHT));

    let forward_cost = forward_pos.iter().find_map(|next_position| {
        traverse(traversed, facing, &next_position, cost + 1, maze, &position)
    });

    let left_cost = left_pos.iter().find_map(|next_position| {
        traverse(
            traversed,
            facing.do_turn(Turn::LEFT),
            &next_position,
            cost + 1001,
            maze,
            &position,
        )
    });

    let right_cost = right_pos.iter().find_map(|next_position| {
        traverse(
            traversed,
            facing.do_turn(Turn::RIGHT),
            &next_position,
            cost + 1001,
            maze,
            &position,
        )
    });

    let list = vec![&forward_cost, &left_cost, &right_cost];

    let min_cost = get_min2(&list);

    let final_return = if let Some(min_cost) = min_cost {
        let mut new_path: Vec<Position> = Vec::new();

        if let Some((_, position)) = forward_cost.filter(|(c, _)| *c == min_cost) {
            new_path.push(forward_pos.unwrap());
            new_path.extend(position);
        }
        if let Some((_, position)) = left_cost.filter(|(c, _)| *c == min_cost) {
            new_path.push(left_pos.unwrap());
            new_path.extend(position);
        }
        if let Some((_, position)) = right_cost.filter(|(c, _)| *c == min_cost) {
            new_path.push(right_pos.unwrap());
            new_path.extend(position);
        }
        Some((min_cost, new_path))
    } else {
        None
    };
    println!(
        "{:?} {:?} {:?} {} {:?}",
        position, last_position, facing, cost, final_return
    );
    final_return
}

fn get_min2(list: &Vec<&Option<(i32, Vec<Position>)>>) -> Option<i32> {
    //let mut index: Option<usize> = None;
    let mut min_cost = None;

    for position in list.iter() {
        if let Some((cost, _)) = position {
            if min_cost.is_none() {
                min_cost = Some(*cost);
            } else if *cost < min_cost.unwrap() {
                min_cost = Some(*cost);
            }
        }
    }
    min_cost
}

fn get_min3(list: &Vec<&Option<(i32, Vec<Position>)>>) -> Option<usize> {
    let mut index: Option<usize> = None;
    let mut min_cost = i32::MAX;

    for (i, position) in list.iter().enumerate() {
        if let Some((cost, _)) = position {
            if *cost < min_cost {
                index = Some(i);
                min_cost = *cost;
            }
        }
    }
    index
}

// fn get_min<'a>(
//     a: &'a Option<(i32, Vec<Position>)>,
//     b: &'a Option<(i32, Vec<Position>)>,
// ) -> &Option<(i32, Vec<Position>)> {
//     if a.is_none() {
//         b
//     } else if b.is_none() {
//         a
//     } else {
//         a

//         let a_val = a.map(|t| t.0).unwrap();
//         let b_val = b.map(|t| t.0).unwrap();
//         if a_val< b_val {
//             a
//         } else {
//             b
//         }
//     }
// }

fn read_data() -> Maze {
    let mut walls: HashSet<Position> = HashSet::new();
    let mut start: Option<Position> = None;
    let mut end: Option<Position> = None;

    let file = File::open("data/day16.txt").expect("File not found");
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
            } else if c == 'S' {
                start = Some(Position {
                    x: x as i32,
                    y: y as i32,
                });
            } else if c == 'E' {
                end = Some(Position {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    Maze {
        walls,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

struct Maze {
    walls: HashSet<Position>,
    start: Position,
    end: Position,
}

impl Maze {
    fn move_forward(&self, position: &Position, facing: &Facing) -> Option<Position> {
        let new_position = match facing {
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
        };
        if self.walls.contains(&new_position) {
            None
        } else {
            Some(new_position)
        }
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

enum Turn {
    LEFT,
    RIGHT,
}

impl Facing {
    fn do_turn(&self, turn: Turn) -> Self {
        match turn {
            Turn::LEFT => match self {
                Facing::NORTH => Facing::WEST,
                Facing::WEST => Facing::SOUTH,
                Facing::SOUTH => Facing::EAST,
                Facing::EAST => Facing::NORTH,
            },
            Turn::RIGHT => match self {
                Facing::NORTH => Facing::EAST,
                Facing::EAST => Facing::SOUTH,
                Facing::SOUTH => Facing::WEST,
                Facing::WEST => Facing::NORTH,
            },
        }
    }
}
