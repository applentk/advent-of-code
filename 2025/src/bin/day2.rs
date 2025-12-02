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
            let k = i.to_string().chars().collect::<Vec<char>>();
            let mut valid = true;

            if k.len() % 2 != 0 {
                continue;
            }

            for j in 0..k.len()/2 {
                if k[j] != k[k.len()/2 + j] {
                    valid = false;
                }
            }

            if valid {
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
            let t = i
                .to_string()
                .chars()
                .collect::<Vec<char>>();

            for j in 1..=t.len()/2 {
                let mut valid = true;

                if t.len() % j != 0 {
                    continue;
                }
                
                let tt = &t[0..j];
                for k in (0..t.len()).step_by(j) {
                    if tt != &t[k..k+j] {
                        valid = false;
                        break;
                    }
                }

                if valid {
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