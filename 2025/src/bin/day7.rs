use std::{collections::HashSet, *};

fn part_1(inp: &Vec<&str>) {
    let a = inp
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let x = 1;
    let y = a[0].iter().position(|&x| x == 'S').unwrap();

    let mut cnt = 0;
    let mut st = vec![(x, y)];
    let mut has_in = HashSet::new();

    while !st.is_empty() {
        let (x, y) = st.pop().unwrap();
        
        if x+1 == a.len() {
            continue;
        }

        if has_in.contains(&(x, y)) {
            continue;
        }
        has_in.insert((x, y));
        
        if x+1 < a.len() && a[x+1][y] == '^' {
            st.push((x+1, y+1));
            st.push((x+1, y-1));
            cnt += 1;
        } else {
            st.push((x+1, y));
        }
    }

    println!("{cnt}");
}

fn dfs(x: usize, y: usize, a: &Vec<Vec<char>>, dp: &mut Vec<Vec<i64>>) -> i64 {
    if y == a[0].len() {
        return 0;
    }

    if x+1 == a.len() {
        return 1;
    }

    if dp[x][y] != -1 {
        return dp[x][y];
    }

    let mut cnt = 0;
    if a[x+1][y] == '^' {
        cnt += dfs(x+1, y+1, a, dp) + dfs(x+1, y-1, a, dp);
    } else {
        cnt += dfs(x+1, y, a, dp);
    }
    
    dp[x][y] = cnt;
    cnt
}

fn part_2(inp: &Vec<&str>) {
    let  a = inp
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let n = a.len();
    let m = a[0].len();

    let x = 1;
    let y = a[0].iter().position(|&x| x == 'S').unwrap();
    let mut dp = (0..n).map(|_| vec![-1; m]).collect();

    println!("{}", dfs(x, y, &a, &mut dp));
}

fn main() -> io::Result<()> {
    let file_path = "src/inputs/day7.in";
    let input = fs::read_to_string(&file_path)?;

    let inputs: Vec<&str> = input.lines().collect();

    part_1(&inputs);
    part_2(&inputs);

    Ok(())
}