use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {


    let file = File::open("data/day1_1.txt").expect("File not found");
    let reader = BufReader::new(file);
    let (mut v1,mut v2) : (Vec<i32>,Vec<i32>) = reader
        .lines()
        .map(|l| l.expect("Couldnt read the file?"))
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|f| f.parse::<i32>().expect("Couldnt parse numbers"))
                .collect::<Vec<i32>>()
        })
        .map(|v| (v[0],v[1]))
        .unzip();

    v1.sort();
    v2.sort();

    let result1 = v1.iter().zip(&v2).map(|x| (x.0-x.1).abs()).sum::<i32>();

    println!("result1 {}",result1);

    let result2 = v1.iter().map(|x| similarity_score(x, &v2)).sum::<i32>();
    println!("result2 {}", result2);


}

fn similarity_score(i : &i32, vec : &Vec<i32>) -> i32
{
    let count = vec.iter()
    .filter(|v| *v==i)
    .count();

     i32::try_from(count).unwrap() * i
}
