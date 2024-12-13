use std::collections::HashSet;

use std::fmt::{self, Debug};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    prob1();
}
fn prob1() {
    let file = File::open("data/day12.txt").expect("File not found");

    let reader = BufReader::new(file);
    let data = reader
        .lines()
        .map(|l| l.expect("Couldnt read the file?"))
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let garden = Garden { data };
    let mut plotted: HashSet<Plot> = HashSet::new();

    let mut result = 0;
    let mut result2 = 0;

    while let Some(next_free_plot) = find_next_unmapped(&plotted, &garden) {
        let mut region: HashSet<Plot> = HashSet::new();
        find_region(next_free_plot, &mut region, &garden);

        let perim: u32 = find_perimiter(&region, &garden);
        let fences = find_fences(&region, &garden);
        assert_eq!(fences.len(), perim as usize);
        let sides = find_sides(&fences);
        let no_sides = sides.len();
        println!("sides {:?} {:?}", sides, fences);

        // println!(
        //     "area {} perimeter {} region {:?}",
        //     region.len(),
        //     perim,
        //     region,
        // );

        result += perim * region.len() as u32;
        result2 += no_sides * region.len();
        plotted.extend(region.into_iter());
        //println!("plotted {:?}", plotted);
    }
    println!("result {}", result);
    println!("result2 {}", result2);
}

fn find_perimiter(region: &HashSet<Plot>, garden: &Garden) -> u32 {
    let mut perimeter = 0;

    for plot in region {
        if garden.up(plot).is_none() {
            perimeter = perimeter + 1;
        }
        if garden.down(plot).is_none() {
            perimeter = perimeter + 1;
        }
        if garden.left(plot).is_none() {
            perimeter = perimeter + 1;
        }
        if garden.right(plot).is_none() {
            perimeter = perimeter + 1;
        }
    }
    perimeter
}
fn find_fences(region: &HashSet<Plot>, garden: &Garden) -> HashSet<Fence> {
    let mut fences: HashSet<Fence> = HashSet::new();

    for plot in region {
        if garden.up(plot).is_none() {
            fences.insert(Fence {
                x: plot.x,
                y: plot.y,
                plant: '-',
                direction: Direction::UP,
            });
        }
        if garden.down(plot).is_none() {
            fences.insert(Fence {
                x: plot.x,
                y: plot.y + 1,
                plant: '-',
                direction: Direction::DOWN,
            });
        }
        if garden.left(plot).is_none() {
            fences.insert(Fence {
                x: plot.x,
                y: plot.y,
                plant: '|',
                direction: Direction::LEFT,
            });
        }
        if garden.right(plot).is_none() {
            fences.insert(Fence {
                x: plot.x + 1,
                y: plot.y,
                plant: '|',
                direction: Direction::RIGHT,
            });
        }
    }
    fences
}

fn find_sides(fences: &HashSet<Fence>) -> Vec<u32> {
    let mut sides: Vec<u32> = Vec::new();
    let mut covered: HashSet<Fence> = HashSet::new();
    println!("fences -> {:?}", fences);

    for fence in fences {
        if !covered.contains(fence) {
            println!("fence (find_sides){:?}", fence);
            let mut side: HashSet<Fence> = HashSet::new();
            build_side(*fence, &mut side, fences);
            sides.push(side.len() as u32);
            println!("side {:?}", side);
            covered.extend(side.into_iter());
        }
    }

    sides
}

fn build_side(fence: Fence, side: &mut HashSet<Fence>, fences: &HashSet<Fence>) {
    println!("build_side {:?} {:?}", fence, side);
    if fence.plant == '-' {
        let left = if fence.x > 0 {
            Some(Fence {
                x: fence.x - 1,
                y: fence.y,
                plant: '-',
                direction: fence.direction,
            })
        } else {
            None
        };

        let right = Fence {
            x: fence.x + 1,
            y: fence.y,
            plant: '-',
            direction: fence.direction,
        };
        side.insert(fence);
        if fences.contains(&right) && !side.contains(&right) {
            build_side(right, side, fences);
        }
        if let Some(left) = left {
            if fences.contains(&left) && !side.contains(&left) {
                build_side(left, side, fences);
            }
        }
    } else {
        let up: Option<Fence> = if fence.y > 0 {
            Some(Fence {
                x: fence.x,
                y: fence.y - 1,
                plant: '|',
                direction: fence.direction,
            })
        } else {
            None
        };

        let down = Fence {
            x: fence.x,
            y: fence.y + 1,
            plant: '|',
            direction: fence.direction,
        };
        side.insert(fence);
        if fences.contains(&down) && !side.contains(&down) {
            build_side(down, side, fences);
        }
        if let Some(up) = up {
            if fences.contains(&up) && !side.contains(&up) {
                build_side(up, side, fences);
            }
        }
    };
    // side.insert(fence);
}

fn find_region(start_plot: Plot, region: &mut HashSet<Plot>, garden: &Garden) {
    //println!("start plot {:?}", start_plot);
    if region.contains(&start_plot) {
        return;
    }

    let up = garden.up(&start_plot);
    let down = garden.down(&start_plot);
    let left = garden.left(&start_plot);
    let right = garden.right(&start_plot);
    region.insert(start_plot);

    if let Some(plot) = up {
        find_region(plot, region, garden);
    }
    if let Some(plot) = down {
        find_region(plot, region, garden);
    }
    if let Some(plot) = left {
        find_region(plot, region, garden);
    }
    if let Some(plot) = right {
        find_region(plot, region, garden);
    }
}

fn find_next_unmapped(plotted: &HashSet<Plot>, garden: &Garden) -> Option<Plot> {
    let data = &garden.data;

    for (y, line) in data.iter().enumerate() {
        for (x, plant) in line.iter().enumerate() {
            let plot = Plot {
                x,
                y,
                plant: *plant,
            };

            if !plotted.contains(&plot) {
                return Some(plot);
            }
        }
    }
    None
}

struct Garden {
    data: Vec<Vec<char>>,
}
impl Garden {
    fn up(&self, plot: &Plot) -> Option<Plot> {
        if plot.y > 0 {
            self.get_plant(plot.x, plot.y - 1)
                .filter(|new_plant| plot.plant == *new_plant)
                .map(|plant| Plot {
                    x: plot.x,
                    y: plot.y - 1,
                    plant,
                })
        } else {
            None
        }
    }
    fn left(&self, plot: &Plot) -> Option<Plot> {
        if plot.x > 0 {
            self.get_plant(plot.x - 1, plot.y)
                .filter(|new_plant| plot.plant == *new_plant)
                .map(|plant| Plot {
                    x: plot.x - 1,
                    y: plot.y,
                    plant,
                })
        } else {
            None
        }
    }
    fn down(&self, plot: &Plot) -> Option<Plot> {
        if plot.y < self.data.len() - 1 {
            self.get_plant(plot.x, plot.y + 1)
                .filter(|new_plant| plot.plant == *new_plant)
                .map(|plant| Plot {
                    x: plot.x,
                    y: plot.y + 1,
                    plant,
                })
        } else {
            None
        }
    }
    fn right(&self, plot: &Plot) -> Option<Plot> {
        if plot.x < self.data[0].len() - 1 {
            self.get_plant(plot.x + 1, plot.y)
                .filter(|new_plant| plot.plant == *new_plant)
                .map(|plant| Plot {
                    x: plot.x + 1,
                    y: plot.y,
                    plant,
                })
        } else {
            None
        }
    }
    fn get_plant(&self, x: usize, y: usize) -> Option<char> {
        // println!("{x}, {y}");
        if y < self.data.len() && x < self.data[0].len() {
            Some(self.data[y][x])
        } else {
            None
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Plot {
    x: usize,
    y: usize,
    plant: char,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Fence {
    x: usize,
    y: usize,
    plant: char,
    direction: Direction,
}
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

impl fmt::Debug for Plot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(format!("P({},{},{})", &self.x, &self.y, &self.plant).as_str())
    }
}

impl fmt::Debug for Fence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            format!(
                "F({},{},{},{:?})",
                &self.x, &self.y, &self.plant, &self.direction
            )
            .as_str(),
        )
    }
}
