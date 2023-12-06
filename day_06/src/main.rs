use std::{
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

    let mut times = vec![];
    let mut distances = vec![];
    for (ix, line) in input_file_buf
        .lines()
        .enumerate()
        .map(|(ix, line)| (ix, line.expect("Expecting a line here")))
    {
        let numbers = line.split(':').collect::<Vec<&str>>()[1]
            .split_ascii_whitespace()
            .map(|t| t.parse::<u32>().expect("Expecting a number"))
            .collect::<Vec<u32>>();

        if ix == 0 {
            times = numbers;
        } else {
            distances = numbers;
        }
    }

    let mut result = None;
    for (time, distance) in zip(times, distances) {
        let exact = if time % 2 == 0 { true } else { false };

        let mut wins = 0;
        for t in 1..=(time / 2) {
            let r = t * (time - t);
            if r > distance {
                wins += if exact && t == time / 2 { 1 } else { 2 };
            }
        }
        result = Some(result.unwrap_or(1) * wins);
    }

    println!("{}", result.unwrap_or_default());

    Ok(())
}
