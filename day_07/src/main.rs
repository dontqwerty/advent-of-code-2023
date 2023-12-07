use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Error},
    iter::zip,
};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input_filepath = args.get(1).expect("Please specify the input filepath");
    let input_file = File::open(input_filepath)?;
    let input_file_buf = BufReader::new(input_file);

    let mut hands = vec![];
    let mut bids = vec![];
    for line in input_file_buf
        .lines()
        .map(|line| line.expect("Expecting a line here"))
    {
        let line = line
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        hands.push(line[0].to_owned());
        bids.push(line[1].to_owned());
    }

    let mut results: Vec<(Vec<usize>, usize, usize)> = vec![];
    for (hand, bid) in zip(&hands, &bids) {
        let mut card_count: HashMap<char, usize> = HashMap::new();
        for card in hand.chars() {
            if let Some(prev_count) = card_count.insert(card, 1) {
                let current_card_count = card_count
                    .get_mut(&card)
                    .expect("Expected value with this key");
                *current_card_count += prev_count;
            }
        }

        let hand_type: usize = if card_count.values().any(|count| count == &5) {
            0
        } else if card_count.values().any(|count| count == &4) {
            1
        } else if card_count.values().any(|count| count == &3)
            && card_count.values().any(|count| count == &2)
        {
            2
        } else if card_count.values().any(|count| count == &3) {
            3
        } else if card_count.values().filter(|count| count == &&2).count() == 2 {
            4
        } else if card_count.values().any(|count| count == &2) {
            5
        } else {
            6
        };

        let values = String::from("AKQJT98765432");
        results.push((
            hand.chars()
                .map(|c| values.find(c).unwrap())
                .collect::<Vec<usize>>(),
            bid.parse::<usize>().expect("Expected bid to be a number"),
            hand_type,
        ));
    }

    // type: [(hand, bid)]
    let mut same_types: HashMap<usize, Vec<(&Vec<usize>, usize)>> = HashMap::new();
    for ix in 0..results.len() {
        if same_types.contains_key(&results[ix].2) {
            continue;
        }

        let mut same_type = results
            .iter()
            .filter(|r| r.2 == results[ix].2)
            .map(|r| (&r.0, r.1))
            .collect::<Vec<(&Vec<usize>, usize)>>();

        same_type.sort_by_key(|s| s.0);
        same_types.insert(results[ix].2, same_type);
    }

    // type: [(hand, bid)]
    let mut same_types: Vec<(&usize, &Vec<(&Vec<usize>, usize)>)> =
        Vec::from_iter(same_types.iter());
    same_types.sort_by_key(|c| c.0);

    let mut results = vec![];
    for v in same_types.iter() {
        for vv in v.1.iter() {
            results.push(vv.1);
        }
    }
    let mut total = 0;

    for (i, r) in results.iter().enumerate() {
        total += (results.len() - i) * r;
    }

    println!("{}", total);

    Ok(())
}
