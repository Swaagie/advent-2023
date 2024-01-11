use std::fs;

fn main() {
    // read file input
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let lines = input.split('\n').collect::<Vec<_>>();
    let mut result: Vec<i32> = vec![];

    for line in lines {
        let measurement = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .rev() // part 2
            .collect::<Vec<_>>();

        let mut gaps: Vec<Vec<i32>> = vec![measurement.clone()];
        let mut current = measurement;
        while current.iter().filter(|&&x| x != 0).count() > 0 {
            println!("{:?}", current);
            current = current
                .windows(2)
                .map(|vs| {
                    let [x, y] = vs else { unreachable!() };
                    y - x
                })
                .collect();

            gaps.push(current.clone());
        }
        println!("{:?}", gaps);

        let mut gaps = gaps.into_iter().rev().collect::<Vec<_>>();
        for i in 0..gaps.len() - 1 {
            let last = *gaps[i].last().unwrap();
            let next_last = *gaps[i + 1].last().unwrap();
            gaps[i + 1].push(last + next_last);
        }

        result.push(*gaps.last().unwrap().last().unwrap());
    }

    println!("Result {:?}", result.iter().sum::<i32>());
}
