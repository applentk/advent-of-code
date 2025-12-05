use std::*;

fn part_1(inputs: &Vec<&str>) {
    let a = inputs
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let n = inputs.len() as i32;
    let m = inputs[0].len() as i32;

    let count = |x: i32, y: i32| {
        let mut cnt = 0;
        
        for i in -1..=1 {
            for j in -1..=1 {
                let (nx, ny) = (x+i, y+j);  
                if 0 <= nx && nx <= n-1 && 0 <= ny && ny <= m-1 && a[nx as usize][ny as usize] == '@' {
                    cnt += 1;
                }
            }
        }

        cnt - 1
    };

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if a[i as usize][j as usize] == '@' && count(i, j) < 4 {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}

fn part_2(inputs: &Vec<&str>) {
    let mut a = inputs
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let n = inputs.len() as i32;
    let m = inputs[0].len() as i32;

    let count = |x: i32, y: i32, a: &Vec<Vec<char>>| {
        let mut cnt = 0;
        
        for i in -1..=1 {
            for j in -1..=1 {
                let (nx, ny) = (x+i, y+j);
                if 0 <= nx && nx <= n-1 && 0 <= ny && ny <= m-1 && a[nx as usize][ny as usize] == '@' {
                    cnt += 1;
                }
            }
        }   

        cnt - 1
    };

    let mut ans = 0;
    let mut doing = true;

    while doing {
        doing = false;
        for i in 0..n {
            for j in 0..m {
                if a[i as usize][j as usize] == '@' && count(i, j, &a) < 4 {
                    ans += 1;
                    a[i as usize][j as usize] = '.';
                    doing = true;
                }
            }
        }
    }

    println!("{ans}");
}

fn main() -> io::Result<()> {
    let file_path = "src/inputs/day4.in";
    let input = fs::read_to_string(&file_path)?;

    let inputs: Vec<&str> = input.lines().collect();

    part_1(&inputs);
    part_2(&inputs);

    Ok(())
}