use std::*;

fn part_1(inputs: &Vec<&str>) {
    let a = (&inputs[0..inputs.len()-1])
        .iter()
        .map(|x| x
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
        )
        .collect::<Vec<Vec<i64>>>();
        
    let b = inputs[inputs.len()-1]
        .split_whitespace()
        .map(|x| x.chars().nth(0).unwrap())
        .collect::<Vec<char>>();

    let mut ans = 0;
    for j in 0..a[0].len() {
        let mut sum = if b[j] == '+' { 0 } else { 1 };

        for i in 0..inputs.len()-1 {
            if b[j] == '+' {
                sum += a[i][j];
            } else {
                sum *= a[i][j];
            }
        }

        ans += sum;
    }

    println!("{ans}");
}

fn part_2(inputs: &Vec<&str>) {
    let tmp = (&inputs[0..inputs.len()-1])
        .iter()
        .map(|x| x.split_whitespace()
            .collect::<Vec<&str>>()
        )
        .collect::<Vec<Vec<&str>>>();

    let mut mx_len = vec![0; tmp[0].len()];
    for j in 0..tmp[0].len() {
        for i in 0..inputs.len()-1 {
            mx_len[j] = cmp::max(mx_len[j], tmp[i][j].len());
        }
    }

    let mut a = vec![];
    for i in 0..inputs.len()-1 {
        let mut j = 0;
        let mut k = 0;
        
        let mut nums = vec![];
        while k < inputs[i].len() {
            let num = (&inputs[i][k..k + mx_len[j]]).chars().collect::<Vec<char>>();
            nums.push(num);

            k += mx_len[j] + 1;
            j += 1;
        }

        a.push(nums);
    }

    let b = inputs[inputs.len()-1]
        .split_whitespace()
        .map(|x| x.chars().nth(0).unwrap())
        .collect::<Vec<char>>();

    let mut ans = 0;
    for j in 0..a[0].len() {
        let mut sum = if b[j] == '+' { 0 } else { 1 };

        for k in 0..mx_len[j] {
            let mut str = String::new();
            
            for i in 0..a.len() {
                if a[i][j][k] != ' ' {
                    str.push(a[i][j][k]);
                }
            }

            let num = str.parse::<i64>().unwrap();
            if b[j] == '+' {
                sum += num;
            } else {
                sum *= num;
            }
        }

        ans += sum;
    }

    println!("{ans}");
}

fn main() -> io::Result<()> {
    let file_path = "src/inputs/day6.in";
    let input = fs::read_to_string(&file_path)?;

    let inputs: Vec<&str> = input.lines().collect();

    part_1(&inputs);
    part_2(&inputs);

    Ok(())
}