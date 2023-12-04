use std::fs;

fn main() {
    // read file input
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let lines: Vec<&str> = input.split("\n").collect();

    // part 1
    // let mut result: f64 = 0.0;
    // for line in lines {
    //     let game = line.split('|').collect::<Vec<_>>();
    //     let possible = game[0].split(':').collect::<Vec<_>>()[1]
    //         .split_whitespace()
    //         .collect::<Vec<_>>();
    //     let numbers = game[1].split_whitespace().collect::<Vec<_>>();
    //     let mut score = 0.5;

    //     for number in numbers {
    //         match possible.contains(&number) {
    //             true => score = score * 2.0,
    //             false => (),
    //         };
    //     }

    //     if score > 0.5 {
    //         result += score;
    //     }
    // }

    // part 2
    // extra copies, add game 0 and start with game 1 which will be processed once
    let mut dict: Vec<i32> = vec![0, 1];

    for line in lines {
        let game = line.split('|').collect::<Vec<_>>();
        let possible = game[0].split(':').collect::<Vec<_>>();
        let numbers = game[1].split_whitespace().collect::<Vec<_>>();
        let mut id = possible[0].split_whitespace().collect::<Vec<_>>()[1]
            .parse::<usize>()
            .unwrap();
        let possible = possible[1].split_whitespace().collect::<Vec<_>>();
        let current = if dict.len() < id + 1 { 1 } else { dict[id] };

        for number in numbers {
            if possible.contains(&number) {
                id += 1;
                if dict.len() < id + 1 {
                    dict.resize(id + 1, 1);
                }

                dict[id] = dict[id] + 1 * current;
            }
        }
    }

    println!("Result: {}", dict.iter().sum::<i32>());
}
