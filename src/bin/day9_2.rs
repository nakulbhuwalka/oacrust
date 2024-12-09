use std::fs::read_to_string;
fn main() {
    let data = read_to_string("data/day9.txt").unwrap();
    let blocks = data
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();

    let mut disk: Vec<Option<u64>> = Vec::new();
    let mut myfiles: Vec<MyFile> = Vec::new();

    for (index, size) in blocks.iter().enumerate() {
        if index % 2 == 0 {
            let id = (index / 2) as u64;
            let mut file = (0..*size).map(|_| Some(id)).collect::<Vec<Option<u64>>>();
            myfiles.push(MyFile {
                id,
                size: *size,
                position: disk.len(),
            });
            disk.append(&mut file);
        } else {
            let mut file = (0..*size).map(|_| None).collect::<Vec<Option<u64>>>();
            disk.append(&mut file);
        }
    }

    for myfile in myfiles.iter().rev() {
        let next = next_free(&disk, myfile);
        //println!("move {:?} to {:?}", myfile, next);
        if let Some(new_position) = next {
            for i in 0..myfile.size {
                disk[new_position + i as usize] = Some(myfile.id);
                disk[myfile.position + i as usize] = None;
            }
        }
        //print_disk(&disk);
    }

    let result: u64 = disk
        .iter()
        .enumerate()
        .filter_map(|(i, id)| id.map(|x| x * i as u64))
        .map(|x| x)
        .sum();

    println!("{}", result);
}

#[derive(Debug)]
struct MyFile {
    id: u64,
    position: usize,
    size: u64,
}

fn print_disk(disk: &Vec<Option<u64>>) {
    disk.iter()
        .map(|f| match f {
            Some(id) => id.to_string(),
            None => ".".to_string(),
        })
        .for_each(|s| print!("{}", s));
    println!("");
}

fn next_free(disk: &Vec<Option<u64>>, myfile: &MyFile) -> Option<usize> {
    for index in 0..myfile.position {
        if disk[index].is_none() {
            let is_empty = (index + 1..index + myfile.size as usize).all(|e| disk[e].is_none());
            if is_empty {
                return Some(index);
            }
        }
    }
    None
}
