use std::{fs, io};

fn part_1(inputs: &Vec<&str>) {
    let mut k = 50;
    let mut ans = 0;

    for s in inputs {
        let d = s.chars().nth(0).unwrap();
        let total = &s[1..].parse::<i32>().unwrap();

        if d == 'R' {
            k = (k + total) % 100;
        } else {
            k = (((k - total) % 100) + 100) % 100;
        }

        if k == 0 {
            ans += 1;
        }
    }

    println!("{ans}");
}

fn part_2(inputs: &Vec<&str>) {
    let mut k = 50;
    let mut ans = 0;

    for s in inputs {
        let d = s.chars().nth(0).unwrap();
        let total = &s[1..].parse::<i32>().unwrap();

        for _ in 1..=*total {
            if d == 'L' {
                k -= 1;
            } else {
                k += 1;
            }

            k = ((k % 100) + 100) % 100;

            if k == 0 {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}

fn main() -> io::Result<()> {
    let file_path = "src/inputs/day1.in";
    let input = fs::read_to_string(&file_path)?;

    let inputs: Vec<&str> = input.split("\n").collect();

    part_1(&inputs);
    part_2(&inputs);

    Ok(())
}