use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input_filepath = args.get(1).expect("Please specify the input filepath");
    let input_file = File::open(input_filepath)?;
    let input_file_buf = BufReader::new(input_file);

    let mut total_score = 0;
    for line in input_file_buf
        .lines()
        .map(|line| line.expect("Expecting a line here"))
    {
        let line = line.split(": ").collect::<Vec<&str>>()[1]
            .split(" | ")
            .collect::<Vec<&str>>();

        let mut winning_cards: HashMap<u32, u32> = HashMap::new();
        for winning_card in line[0].split_ascii_whitespace() {
            let winning_number = winning_card
                .parse::<u32>()
                .expect("Expected winning number");

            if winning_cards.insert(winning_number, 0).is_some() {
                panic!("Two equal winning numbers found!");
            }
        }

        for card in line[1].split_ascii_whitespace() {
            let number = card.parse::<u32>().expect("Expected winning number");

            if let Some(matching_number) = winning_cards.get_mut(&number) {
                *matching_number = 1;
            }
        }

        let scores = winning_cards
            .iter()
            .map(|(_, v)| v)
            .filter(|v| v > &&0)
            .collect::<Vec<&u32>>();

        if scores.len() > 0 {
            let score = 2_i32.pow(scores.len() as u32 - 1) as u32;
            total_score += score;
        }
    }

    println!("{}", total_score);

    Ok(())
}
