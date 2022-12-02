use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let file = File::open("puzzle_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut elves: Vec<i32> = vec![];
    let mut total: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            elves.push(total);
            total = 0;
            continue;
        }
        total += line.parse::<i32>().unwrap();
    }

    elves.sort();
    elves.reverse();
    println!("{:?}", elves[0]);
    let elves: Vec<f32> = elves.into_iter().map(|x| x as f32).collect::<Vec<f32>>();
    let elves = &elves[0..3];
    println!("{:?}", elves.iter().sum::<f32>() as i32);
    //println!("{:?}", &elves[0..3].iter().sum());
}
