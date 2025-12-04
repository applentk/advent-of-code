use std::{fs, io};

fn part_1(inputs: &Vec<&str>) {
    let s: Vec<&str> = inputs[0].split(",").collect();

    let mut ans = 0;
    for line in s {
        let [l, r] = line
            .split("-")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();

        for i in l..=r {
            let t = i.to_string().chars().collect::<Vec<char>>();
            if t.len() % 2 != 0 {
                continue;
            }

            let (first, second) = t.split_at(t.len()/2); 
            if first == second {
                ans += i;
            }
        }
    }

    println!("{ans}");
}

fn part_2(inputs: &Vec<&str>) {
    let s: Vec<&str> = inputs[0].split(",").collect();

    let mut ans = 0;
    for line in s {
        let [l, r] = line
            .split("-")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();

        for i in l..=r {
            let t: Vec<char> = i.to_string().chars().collect();

            for j in 1..=t.len()/2 {
                if t.len() % j != 0 {
                    continue;
                }

                let chunks: Vec<&[char]> = t
                    .chunks(j)
                    .collect();

                if chunks.iter().all(|x| *x == chunks[0]) {
                   ans += i;
                   break; 
                }
            }
        }
    }

    println!("{ans}");
}

fn main() -> io::Result<()> {
    let file_path = "src/inputs/day2.in";
    let input = fs::read_to_string(&file_path)?;

    let inputs: Vec<&str> = input.split("\n").collect();

    part_1(&inputs);
    part_2(&inputs);

    Ok(())
}