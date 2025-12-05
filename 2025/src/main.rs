// Using this as template
use std::{fs, io};

fn part_1(inputs: &Vec<&str>) {
    
}

fn part_2(inputs: &Vec<&str>) {
    
}

fn main() -> io::Result<()> {
    let file_path = "src/inputs/day.in";
    let input = fs::read_to_string(&file_path)?;

    let inputs: Vec<&str> = input.lines().collect();

    part_1(&inputs);
    part_2(&inputs);

    Ok(())
}