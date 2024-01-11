use std::fs;

fn main() {
    // read file input
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    // Part 1
    // split lines
    // let lines: Vec<&str> = input.split('\n').collect::<Vec<_>>();
    // let time = lines[0]
    //     .split_whitespace()
    //     .map(|x| x.parse::<i32>().unwrap())
    //     .collect::<Vec<_>>();
    // let distance = lines[1]
    //     .split_whitespace()
    //     .map(|x| x.parse::<i32>().unwrap())
    //     .collect::<Vec<_>>();

    // println!("time: {:?}\ndistance: {:?}", time, distance);
    // let mut number_of_records = vec![];

    // for (index, t) in time.iter().enumerate() {
    //     let mut records = vec![];

    //     for delta in 1..*t {
    //         let distance = (t - delta) * delta;
    //         records.push(distance);
    //     }

    //     println!("records: {:?}", records);
    //     let records = records
    //         .iter()
    //         .filter(|&&d| d > distance[index])
    //         .collect::<Vec<_>>();

    //     number_of_records.push(records.len());
    // }

    // Part 2
    let lines: Vec<&str> = input.split('\n').collect::<Vec<_>>();
    let time = lines[0].replace(" ", "").parse::<i64>().unwrap();
    let distance = lines[1].replace(" ", "").parse::<i64>().unwrap();

    let mut records = vec![];

    for delta in 1..time {
        let distance = (time - delta) * delta;
        records.push(distance);
    }

    let records = records
        .iter()
        .filter(|&&d| d > distance)
        .collect::<Vec<_>>();

    println!("Result: {:?}", records.len());
}
