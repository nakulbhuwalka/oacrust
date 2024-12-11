use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    prob1();
}
fn prob1() {
    let file = File::open("data/day10.txt").expect("File not found");

    let reader = BufReader::new(file);
    let data = reader
        .lines()
        .map(|l| l.expect("Couldnt read the file?"))
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let map = Map { data };

    let trailheads = map
        .data
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, c)| Point { x, y, height: *c })
        })
        .filter(|t| t.height == 0)
        .collect::<Vec<Point>>();

    // println!("trailheads {:?}", trailheads);
    println!("{:?}", map.data);
    let trails = trailheads
        .iter()
        .map(|trailhead| fetch_path_count(trailhead, &map))
        .collect::<Vec<usize>>();
    println!("result 1{}", trails.iter().sum::<usize>());

    let trails2: Vec<usize> = trailheads
        .iter()
        .map(|trailhead| fetch_path_count2(trailhead, &map))
        .collect::<Vec<usize>>();
    println!("result 2{}", trails2.iter().sum::<usize>());
}

fn fetch_path_count(trailhead: &Point, map: &Map) -> usize {
    let mut paths: HashSet<(usize, usize)> = HashSet::new();

    find_path(trailhead, map, &mut paths);

    paths.len()
}
fn find_path(current: &Point, map: &Map, paths: &mut HashSet<(usize, usize)>) {
    if current.height == 9 {
        paths.insert((current.x, current.y));
    } else {
        if let Some(new_point) = map.up(current) {
            find_path(&new_point, map, paths)
        };
        if let Some(new_point) = map.down(current) {
            find_path(&new_point, map, paths)
        };
        if let Some(new_point) = map.left(current) {
            find_path(&new_point, map, paths)
        };
        if let Some(new_point) = map.right(current) {
            find_path(&new_point, map, paths)
        };
    }
}

fn fetch_path_count2(trailhead: &Point, map: &Map) -> usize {
    let mut paths: Vec<(usize, usize)> = Vec::new();

    find_path2(trailhead, map, &mut paths);

    paths.len()
}
fn find_path2(current: &Point, map: &Map, paths: &mut Vec<(usize, usize)>) {
    if current.height == 9 {
        paths.push((current.x, current.y));
    } else {
        if let Some(new_point) = map.up(current) {
            find_path2(&new_point, map, paths)
        };
        if let Some(new_point) = map.down(current) {
            find_path2(&new_point, map, paths)
        };
        if let Some(new_point) = map.left(current) {
            find_path2(&new_point, map, paths)
        };
        if let Some(new_point) = map.right(current) {
            find_path2(&new_point, map, paths)
        };
    }
}

struct Map {
    data: Vec<Vec<u32>>,
}
impl Map {
    fn up(&self, point: &Point) -> Option<Point> {
        if point.y > 0 {
            self.get_height(point.x, point.y - 1)
                .filter(|new_height| point.height + 1 == *new_height)
                .map(|height| Point {
                    x: point.x,
                    y: point.y - 1,
                    height,
                })
        } else {
            None
        }
    }
    fn left(&self, point: &Point) -> Option<Point> {
        if point.x > 0 {
            self.get_height(point.x - 1, point.y)
                .filter(|new_height| point.height + 1 == *new_height)
                .map(|height| Point {
                    x: point.x - 1,
                    y: point.y,
                    height,
                })
        } else {
            None
        }
    }
    fn down(&self, point: &Point) -> Option<Point> {
        if point.y < self.data.len() - 1 {
            self.get_height(point.x, point.y + 1)
                .filter(|new_height| point.height + 1 == *new_height)
                .map(|height| Point {
                    x: point.x,
                    y: point.y + 1,
                    height,
                })
        } else {
            None
        }
    }
    fn right(&self, point: &Point) -> Option<Point> {
        if point.x < self.data[0].len() - 1 {
            self.get_height(point.x + 1, point.y)
                .filter(|new_height| point.height + 1 == *new_height)
                .map(|height| Point {
                    x: point.x + 1,
                    y: point.y,
                    height,
                })
        } else {
            None
        }
    }
    fn get_height(&self, x: usize, y: usize) -> Option<u32> {
        // println!("{x}, {y}");

        if y < self.data.len() && x < self.data[0].len() {
            Some(self.data[y][x])
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    height: u32,
}
