use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input_filepath = args.get(1).expect("Please specify the input filepath");
    let input_file = File::open(input_filepath)?;
    let input_file_buf = BufReader::new(input_file);

    let mut total = 0;

    for line in input_file_buf
        .lines()
        .map(|line| line.expect("Expecting a line here"))
    {
        let numbers = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let numbers = numbers
            .iter()
            .map(|n| n.parse::<i32>().expect("Expecting a number"))
            .collect::<Vec<i32>>();

        let mut done = false;
        let mut results = vec![numbers];
        while !done {
            results.push(diff(results.last().unwrap()));
            if results.last().unwrap().iter().all(|n| *n == 0) {
                done = true;
            }
        }

        let mut foo = vec![];
        for (i, r) in results.iter().enumerate().rev() {
            if i == results.len() - 1 {
                foo.push(0);
                continue;
            }
            foo.push(r.last().unwrap() + foo.last().unwrap())
        }

        total += foo.last().unwrap();
    }

    println!("{}", total);

    Ok(())
}

fn diff(numbers: &Vec<i32>) -> Vec<i32> {
    let mut results = vec![];
    for (i, n) in numbers.iter().enumerate() {
        if i == numbers.len() - 1 {
            break;
        }

        dbg!(numbers[i + 1]);
        dbg!(n);
        results.push(numbers[i + 1] - n);
    }
    results
}
