use std::fs;

fn main() {
    let numbers: Vec<(&str, char)> = vec![
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    // read file input
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    // split input into lines
    let lines: Vec<&str> = input.split("\n").collect();
    let mut result = 0;

    for line in lines {
        let mut digits = numbers
            .iter()
            .flat_map(|(alpha, digit)| {
                let mut result = vec![];

                // First
                if let Some(i) = line.find(alpha) {
                    result.push((i, digit));
                }

                // Last
                if let Some(i) = line.rfind(alpha) {
                    result.push((i, digit));
                }

                result
            })
            .collect::<Vec<_>>();

        digits.sort_by(|a, b| a.0.cmp(&b.0));

        // Nasty concat
        let mut value = String::from("");
        value.push(*digits[0].1);
        value.push(*digits.last().unwrap().1);
        result += value.parse::<i32>().unwrap();
    }

    println!("Result: {}", result);
}
