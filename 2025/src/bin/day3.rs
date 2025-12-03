use std::*;

fn part_1(inputs: &Vec<&str>) {
    let mut ans = 0;

    for s in inputs {
        let mut a: Vec<(char, usize)> = s
            .chars()
            .enumerate()
            .map(|(index, value)| (value, index))
            .collect();

        a.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

        let mut done = false;
        for i in 0..a.len() {
            if done { break; }
            for j in 0..a.len() {
                if i == j { continue; }
                if a[i].1 < a[j].1 {
                    ans += format!("{}{}", a[i].0, a[j].0).parse::<i32>().unwrap();
                    done = true;
                    break;
                }
            }
        }
    }

    println!("{ans}");
}

fn part_2(inputs: &Vec<&str>) {
    let mut ans = 0;

    for s in inputs {
        let mut b = vec!['z'; s.len()];

        let mut cnt = 0;
        let mut best = 0;

        while cnt != 12 {
            let mut select: i32 = -1;
            let mut select_char = 'z';

            for (i, c) in s.chars().enumerate() {
                let mut bb = b.clone();
                bb[i] = c;

                let mut num_str = String::new();
                for d in bb {
                    if d != 'z' {
                        num_str.push(d);
                    }
                }

                let num = num_str.parse::<i64>().unwrap();
                if num > best {
                    best = num;
                    select = i as i32;
                    select_char = c;
                }
            }

            b[select as usize] = select_char;
            cnt += 1;
        }

        ans += best;
    }

    println!("{ans}");
}

fn main() -> io::Result<()> {
    let file_path = "src/inputs/day3.in";
    let input = fs::read_to_string(&file_path)?;

    let inputs: Vec<&str> = input.lines().collect();

    part_1(&inputs);
    part_2(&inputs);

    Ok(())
}