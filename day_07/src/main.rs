use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Error},
    iter::zip,
};

static CARD_MAP_0: &str = "AKQJT98765432";
static CARD_MAP_1: &str = "AKQT98765432J";

#[derive(Debug)]
struct Hand {
    cards: Vec<usize>,
    bid: usize,
    strength: usize,
}

impl Hand {
    fn from_line(line: &str, with_jokers: bool) -> Self {
        let cards_and_bid = line.split_ascii_whitespace().collect::<Vec<&str>>();

        let card_map = if with_jokers { CARD_MAP_1 } else { CARD_MAP_0 };

        let cards = cards_and_bid[0]
            .chars()
            .map(|c| card_map.find(c).unwrap())
            .collect::<Vec<usize>>();
        let bid = cards_and_bid[1].parse::<usize>().unwrap();

        let mut card_counts = HashMap::new();
        for card in cards.iter() {
            if card_counts.get(card).is_some() {
                continue;
            }
            let count = cards.iter().filter(|c| *c == card).count();
            card_counts.insert(card, count);
        }

        let counts = if with_jokers {
            let jokers_count = *card_counts.get(&12).unwrap_or(&0);
            if jokers_count == 5 {
                vec![5]
            } else {
                let mut counts = card_counts
                    .iter()
                    .filter(|cc| **cc.0 != 12)
                    .map(|cc| *cc.1)
                    .collect::<Vec<usize>>();
                *counts.iter_mut().max().unwrap() += jokers_count;

                counts
            }
        } else {
            card_counts.iter().map(|cc| *cc.1).collect::<Vec<usize>>()
        };

        let strength = if counts.iter().any(|c| *c == 5) {
            0
        } else if counts.iter().any(|c| *c == 4) {
            1
        } else if counts.iter().any(|c| *c == 3) && counts.iter().any(|c| *c == 2) {
            2
        } else if counts.iter().any(|c| *c == 3) {
            3
        } else if counts.iter().filter(|c| **c == 2).count() == 2 {
            4
        } else if counts.iter().any(|c| *c == 2) {
            5
        } else {
            6
        };

        Self {
            cards,
            bid,
            strength,
        }
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input_filepath = args.get(1).expect("Please specify the input filepath");
    let input_file = File::open(input_filepath)?;

    let input_file_buf = BufReader::new(input_file);

    let mut hands_0 = vec![];
    let mut hands_1 = vec![];
    for line in input_file_buf
        .lines()
        .map(|line| line.expect("Expecting a line here"))
    {
        hands_0.push(Hand::from_line(&line, false));
        hands_1.push(Hand::from_line(&line, true));
    }

    hands_0.sort_by_key(|h| (h.strength, h.cards.clone()));
    hands_1.sort_by_key(|h| (h.strength, h.cards.clone()));

    let hands_count = hands_0.len();
    let mut total_0 = 0;
    let mut total_1 = 0;
    for (ix, (hand_0, hand_1)) in zip(hands_0, hands_1).enumerate() {
        total_0 += hand_0.bid * (hands_count - ix);
        total_1 += hand_1.bid * (hands_count - ix);
    }

    println!("{}", total_0);
    println!("{}", total_1);

    Ok(())
}
