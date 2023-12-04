use std::fs;

fn main() {
    // read file input
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    // split input into lines
    let lines: Vec<&str> = input.split("\n").collect();
    let mut result = 0;

    for line in lines {
        let game = line.split(":").collect::<Vec<_>>();

        // Part 1
        // let mut possible = true;
        // let id = game[0].parse::<i32>().unwrap();
        // let mut subsets = game[1].split(";").peekable();
        //
        // while possible && subsets.peek().is_some() {
        //     let subset = subsets.next().unwrap();
        //     for value in subset.split(",") {
        //         let split = value.split_whitespace().collect::<Vec<_>>();
        //         let number = split[0].parse::<i32>().unwrap();
        //         match split[1] {
        //             "red" => {
        //                 if number > 12 {
        //                     possible = false;
        //                 }
        //             }
        //             "green" => {
        //                 if number > 13 {
        //                     possible = false;
        //                 }
        //             }
        //             "blue" => {
        //                 if number > 14 {
        //                     possible = false;
        //                 }
        //             }
        //             _ => {
        //                 possible = false;
        //             }
        //         }
        //     }
        // }

        // if possible {
        //     result += id;
        // }

        let mut red = vec![];
        let mut green = vec![];
        let mut blue = vec![];

        for subset in game[1].split(";") {
            for value in subset.split(",") {
                let split = value.split_whitespace().collect::<Vec<_>>();
                let number = split[0].parse::<i32>().unwrap();
                match split[1] {
                    "red" => red.push(number),
                    "green" => green.push(number),
                    "blue" => blue.push(number),
                    _ => {}
                }
            }
        }

        let max_red = red.iter().fold(0, |acc, x| acc.max(*x));
        let max_green = green.iter().fold(0, |acc, x| acc.max(*x));
        let max_blue = blue.iter().fold(0, |acc, x| acc.max(*x));

        result += max_red * max_green * max_blue;
    }

    println!("Result: {}", result);
}
