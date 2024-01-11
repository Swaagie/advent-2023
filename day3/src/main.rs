use std::fs;

fn main() {
    // read file input
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    // find index of numbers in string
    let column_width: i32 = (input.split('\n').next().unwrap().len() + 1) as i32;

    // Part 1
    // let mut locations: Vec<usize> = vec![];
    // for (index, c) in input.chars().enumerate() {
    //     if c != '.' && !c.is_numeric() && c != '\n' {
    //         let mut location = vec![
    //             index - column_width - 1,
    //             index - column_width,
    //             index - column_width + 1,
    //             index - 1,
    //             index + 1,
    //             index + column_width - 1,
    //             index + column_width,
    //             index + column_width + 1,
    //         ];

    //         locations.append(&mut location);
    //     }
    // }

    // let mut slice_start = 0;
    // let mut number_locations = vec![];
    // let chars = input.chars().collect::<Vec<char>>();
    // for i in 0..chars.len() {
    //     if !chars[i].is_numeric() {
    //         slice_start = i + 1;
    //         continue;
    //     }

    //     if !chars[i + 1].is_numeric() {
    //         println!("found range, {} to {}", slice_start, i);
    //         for j in slice_start..i + 1 {
    //             if locations.contains(&j) {
    //                 println!("{} is in locations for {}", &input[slice_start..i + 1], j);
    //                 number_locations.push(input[slice_start..i + 1].parse::<i32>().unwrap());
    //                 break;
    //             }
    //         }

    //         slice_start = i + 1;
    //     }
    // }

    // println!("Result: {}", number_locations.iter().sum::<i32>());

    // Part 2
    let mut slice_start = 0;
    let mut numbers = vec![];
    let chars = input.chars().collect::<Vec<char>>();
    let mut potential_ratio = vec![];

    for i in 0..chars.len() {
        if !chars[i].is_numeric() {
            slice_start = i + 1;
            continue;
        }

        if !chars[i + 1].is_numeric() {
            println!("found range, {} to {}", slice_start, i);
            let mut locations = vec![];

            let start = slice_start as i32;
            let end = i as i32;
            locations.append(&mut ((start - column_width - 1)..(end - column_width + 2)).collect());
            locations.append(&mut ((start - 1)..(end + 2)).collect());
            locations.append(&mut ((start + column_width - 1)..(end + column_width + 2)).collect());

            for l in locations {
                if l > 0 && l < (chars.len() as i32) && chars[l as usize] == '*' {
                    let found_prev_ratio = potential_ratio.iter().find(|(index, _)| l == *index);

                    match found_prev_ratio {
                        Some((_, number)) => {
                            numbers.push(number * input[slice_start..i + 1].parse::<i32>().unwrap())
                        }
                        None => potential_ratio
                            .push((l, input[slice_start..i + 1].parse::<i32>().unwrap())),
                    }

                    break;
                }
            }

            slice_start = i + 1;
        }
    }

    println!("Result: {}", numbers.iter().sum::<i32>());
}
