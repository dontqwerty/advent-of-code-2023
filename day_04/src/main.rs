use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn process_card(card: &Vec<String>) -> usize {
    let mut winning_numbers = HashMap::new();

    for winning_number in card[0].split_ascii_whitespace() {
        let winning_number = winning_number
            .parse::<u32>()
            .expect("Expected winning number");

        if winning_numbers.insert(winning_number, false).is_some() {
            panic!("Two equal winning numbers found!");
        }
    }

    for number in card[1].split_ascii_whitespace() {
        let number = number.parse::<u32>().expect("Expected winning number");

        if let Some(won) = winning_numbers.get_mut(&number) {
            *won = true;
        }
    }

    winning_numbers.iter().filter(|(_, v)| **v).count()
}

fn process_deck(deck: Vec<Vec<String>>) -> (usize, usize) {
    let mut cards_copies = HashMap::new();

    for card_ix in 0..deck.len() {
        cards_copies.insert(card_ix, 1);
    }

    let mut total_wins = 0;

    for (card_ix, card) in deck.iter().enumerate() {
        let card_wins = process_card(card);

        if card_wins > 0 {
            total_wins += 2_u32.pow(card_wins as u32 - 1)
        }

        let card_copies = cards_copies
            .get(&card_ix)
            .expect("Expected an entry with this card index")
            .to_owned();

        for win in 0..card_wins {
            if let Some(prev_copies) = cards_copies.get_mut(&(card_ix + 1 + win)) {
                *prev_copies += card_copies;
            }
        }
    }

    (total_wins as usize, cards_copies.values().sum())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input_filepath = args.get(1).expect("Please specify the input filepath");
    let input_file = File::open(input_filepath)?;
    let input_file_buf = BufReader::new(input_file);

    let mut deck = vec![];
    for line in input_file_buf
        .lines()
        .map(|line| line.expect("Expecting a line here"))
    {
        let line = line.split(": ").collect::<Vec<&str>>()[1]
            .split(" | ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        deck.push(line);
    }

    let (wins, cards) = process_deck(deck);

    println!("{}", wins);
    println!("{}", cards);

    Ok(())
}
