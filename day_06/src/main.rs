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
    let mut time = 0;
    let mut distance = 0;
    for (ix, line) in input_file_buf
        .lines()
        .enumerate()
        .map(|(ix, line)| (ix, line.expect("Expecting a line here")))
    {
        let line = line.split(':').collect::<Vec<&str>>()[1];
        let numbers = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<u64>().expect("Expecting a number"))
            .collect::<Vec<u64>>();
        let number = line
            .replace(" ", "")
            .parse::<u64>()
            .expect("Expecting a number");

        if ix == 0 {
            times = numbers;
            time = number;
        } else {
            distances = numbers;
            distance = number;
        }
    }

    let result_0 = foo(&times, &distances);
    let result_1 = foo(&[time], &[distance]);

    println!("{}", result_0);
    println!("{}", result_1);

    Ok(())
}

fn foo(times: &[u64], distances: &[u64]) -> u64 {
    let mut result = None;
    for (time, distance) in zip(times, distances) {
        let exact = if time % 2 == 0 { true } else { false };
        let mut wins: u64 = 0;
        for t in 1..=(time / 2) {
            let r = t * (time - t);
            if r > *distance {
                wins += if exact && t == time / 2 { 1 } else { 2 };
            }
        }
        result = Some(result.unwrap_or(1) * wins);
    }

    result.unwrap_or_default()
}
