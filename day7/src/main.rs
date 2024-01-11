use core::num;
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CARDS {
    FIVE = 100,
    FOUR = 90,
    FULLHOUSE = 80,
    THREE = 70,
    TWOPAIR = 60,
    PAIR = 50,
    HIGHCARD = 40,
}

fn main() {
    // read file input
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let card_dict = "AKQT98765432J";

    // Part 1
    // split lines
    let lines: Vec<&str> = input.split('\n').collect::<Vec<_>>();
    let mut stack = vec![];

    for line in lines {
        let d = line.split_whitespace().collect::<Vec<_>>();
        let origin_cards = d[0];
        let mut cards = origin_cards.chars().collect::<Vec<_>>();
        let bid = d[1].parse::<i32>().unwrap();

        cards.sort_by(|a, b| card_dict.find(*a).cmp(&card_dict.find(*b)));

        // discover card types
        let mut i = 0;
        let mut final_set = (CARDS::HIGHCARD, origin_cards);
        let mut curr = 1;

        while i < 4 {
            let equal = cards[i] == cards[i + 1];
            if equal && cards[i] != 'J' {
                curr += 1;
            }

            if i == 3 || !equal {
                match curr {
                    5 => {
                        final_set = (CARDS::FIVE, origin_cards);
                        break;
                    }
                    4 => {
                        final_set = (CARDS::FOUR, origin_cards);
                        break;
                    }
                    3 => {
                        if final_set.0 == CARDS::PAIR {
                            final_set = (CARDS::FULLHOUSE, origin_cards);
                        } else {
                            final_set = (CARDS::THREE, origin_cards);
                        }
                        curr = 1;
                    }
                    2 => {
                        if final_set.0 == CARDS::THREE {
                            final_set = (CARDS::FULLHOUSE, origin_cards);
                        } else if final_set.0 == CARDS::PAIR {
                            final_set = (CARDS::TWOPAIR, origin_cards);
                        } else {
                            final_set = (CARDS::PAIR, origin_cards);
                        }
                        curr = 1;
                    }
                    _ => {}
                }
            }

            i += 1;
        }

        println!("{:?} {:?}", cards, final_set);
        stack.push((final_set, bid));
    }

    // brute force J
    for i in 0..stack.len() {
        let s = &mut stack[i];
        if !s.0 .1.contains('J') {
            continue;
        }

        let j = s.0 .1.chars().filter(|&c| c == 'J').count();

        println!("Found J: {}, curr: {:?}, type: {:?}", j, s.0 .1, s.0 .0);
        for _ in 0..j {
            match s.0 .0 {
                CARDS::HIGHCARD => {
                    s.0 .0 = CARDS::PAIR;
                }
                CARDS::PAIR => {
                    s.0 .0 = CARDS::THREE;
                }
                CARDS::TWOPAIR => {
                    s.0 .0 = CARDS::FULLHOUSE;
                }
                CARDS::THREE => {
                    s.0 .0 = CARDS::FOUR;
                }
                CARDS::FOUR => {
                    s.0 .0 = CARDS::FIVE;
                }
                _ => {}
            }
        }
        println!("Upgrade {:?}", s.0 .0);
    }

    stack.sort_by(|a, b| {
        // type is identical
        if a.0 .0 == b.0 .0 {
            let cards_a = a.0 .1.chars().map(|c| card_dict.find(c).unwrap());
            let cards_b = b.0 .1.chars().map(|c| card_dict.find(c).unwrap());
            cards_b.cmp(cards_a)
        } else {
            // Rand different type, see enum
            a.0 .0.cmp(&b.0 .0)
        }
    });

    stack.iter().for_each(|c| println!("{:?}", c));
    println!(
        "Result: {:?}",
        stack
            .iter()
            .enumerate()
            .fold(0, |acc, s| acc + s.1 .1 * (s.0 as i32 + 1))
    );
}
