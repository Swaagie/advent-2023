use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Direction<'a> {
    l: &'a str,
    r: &'a str,
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = second;
    let mut min = first;

    if first >= second {
        max = first;
        min = second;
    }

    loop {
        let remain = max % min;

        if remain == 0 {
            return min;
        }

        max = min;
        min = remain;
    }
}

fn main() {
    // read file input
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut lines = input.split('\n').collect::<Vec<_>>();
    let seq = lines.remove(0).chars().collect::<Vec<_>>();

    let mut dict: HashMap<&str, Direction> = HashMap::new();

    for line in &lines {
        let keys = line.split('=').collect::<Vec<_>>();
        let directions = keys[1].split(',').collect::<Vec<_>>();

        let left = directions[0].trim();
        let right = directions[1].trim();

        dict.insert(
            keys[0].trim(),
            Direction {
                l: &left[1..],
                r: &right[..right.len() - 1],
            },
        );
    }

    // let mut current = "AAA";
    let start = dict
        .keys()
        .filter(|&&x| x.chars().nth(2) == Some('A'))
        .collect::<Vec<_>>();

    // while current != "ZZZ" {
    //     let direction = dict.get(current).unwrap();

    //     match seq[steps % seq.len()] {
    //         'L' => current = direction.l,
    //         'R' => current = direction.r,
    //         _ => (),
    //     }

    //     steps += 1;
    // }

    let mut found_steps = vec![];
    for s in start {
        let mut steps: usize = 0;
        let mut current = s;

        while current.chars().nth(2) != Some('Z') {
            let direction = dict.get(current).unwrap();

            match seq[steps % seq.len()] {
                'L' => current = &direction.l,
                'R' => current = &direction.r,
                _ => (),
            }

            steps += 1;
        }

        found_steps.push(steps);
    }

    // Loopie loopie loop
    println!("Result {:?}", found_steps);
    println!(
        "LCM {:?}",
        found_steps.iter().fold(1, |acc, x| lcm(acc, *x))
    );
}
