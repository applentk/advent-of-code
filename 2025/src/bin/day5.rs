use std::*;

fn part_1(inputs: &Vec<&str>) {
    let mut i = 0;
    let mut a = vec![];

    while inputs[i] != "" {
        let [l, r] = inputs[i]
            .split("-")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();
        
        a.push((l, r));
        i += 1;
    }

    i += 1;

    let mut b = vec![];
    while i < inputs.len() {
        b.push(inputs[i].parse::<i64>().unwrap());
        i += 1;
    }

    let mut ans = 0;
    for x in b {
        let mut is_fresh = false;
        for (l, r) in &a {
            if *l <= x && x <= *r {
                is_fresh = true;
                break;
            }
        }

        if is_fresh {
            ans += 1;
        }
    }    

    println!("{ans}");
}

fn part_2(inputs: &Vec<&str>) {
    let mut i = 0;
    let mut a = vec![];

    while inputs[i] != "" {
        let [l, r] = inputs[i]
            .split("-")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();
        
        a.push((l, r));
        i += 1;
    }

    a.sort();

    let is_overlap = |a: (i64, i64), b: (i64, i64)| {
        cmp::max(a.0, b.0) <= cmp::min(a.1, b.1)
    };

    let merge = |a: (i64, i64), b: (i64, i64)| {
        (cmp::min(a.0, b.0), cmp::max(a.1, b.1))
    };

    let mut k = a[0];
    let mut ranges = vec![];
    for i in 1..a.len() {
        if is_overlap(k, a[i]) {
            k = merge(k, a[i]);
        } else {
            ranges.push(k);
            k = a[i];
        }
    }
    ranges.push(k);

    let mut ans = 0;
    for (l, r) in ranges {
        ans += r-l+1;
    }
    
    println!("{ans}");
}

fn main() -> io::Result<()> {
    let file_path = "src/inputs/day5.in";
    let input = fs::read_to_string(&file_path)?;

    let inputs: Vec<&str> = input.lines().collect();

    part_1(&inputs);
    part_2(&inputs);

    Ok(())
}