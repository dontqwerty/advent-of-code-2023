use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Error},
    iter::zip,
};

const MIN_LITERAL_DIGIT_LEN: usize = 3;

fn get_digit(literal_digit: &[u8]) -> Option<u32> {
    const ONE: &str = "one";
    const TWO: &str = "two";
    const THREE: &str = "three";
    const FOUR: &str = "four";
    const FIVE: &str = "five";
    const SIX: &str = "six";
    const SEVEN: &str = "seven";
    const EIGHT: &str = "eight";
    const NINE: &str = "nine";

    let literal_digit =
        std::str::from_utf8(literal_digit).expect("Should only contain valid UTF-8 characters");

    match literal_digit {
        ONE => Some(1),
        TWO => Some(2),
        THREE => Some(3),
        FOUR => Some(4),
        FIVE => Some(5),
        SIX => Some(6),
        SEVEN => Some(7),
        EIGHT => Some(8),
        NINE => Some(9),
        _ => None,
    }
}

fn get_first_digit_from_literal(literal_digit: &[u8]) -> Option<u32> {
    let n = literal_digit.len();

    if n < MIN_LITERAL_DIGIT_LEN {
        return None;
    }

    for i in 0..n - MIN_LITERAL_DIGIT_LEN + 1 {
        let literal_digit_slice = &literal_digit[i..n];
        if let Some(digit) = get_digit(literal_digit_slice) {
            return Some(digit);
        }
    }

    None
}

fn get_last_digit_from_literal(literal_digit: &[u8]) -> Option<u32> {
    let n = literal_digit.len();

    if n < MIN_LITERAL_DIGIT_LEN {
        return None;
    }

    for i in (MIN_LITERAL_DIGIT_LEN..n + 1).rev() {
        let literal_digit_slice = &literal_digit[0..i];
        if let Some(digit) = get_digit(literal_digit_slice) {
            return Some(digit);
        }
    }

    None
}

fn get_first_digit(input: &str) -> Option<u32> {
    let mut literal_digit = vec![];
    for (char, byte) in zip(input.chars(), input.bytes()) {
        if char.is_ascii_digit() {
            return Some(char.to_digit(10).expect("This char should be a digit"));
        } else {
            literal_digit.push(byte);
            if let Some(digit) = get_first_digit_from_literal(literal_digit.as_slice()) {
                return Some(digit);
            }
        }
    }

    None
}

fn get_last_digit(input: &str) -> Option<u32> {
    let mut literal_digit = vec![];
    for (char, byte) in zip(input.chars().rev(), input.bytes().rev()) {
        if char.is_ascii_digit() {
            return Some(char.to_digit(10).expect("This char should be a digit"));
        } else {
            literal_digit.insert(0, byte);
            if let Some(digit) = get_last_digit_from_literal(literal_digit.as_slice()) {
                return Some(digit);
            }
        }
    }

    None
}

fn get_number(input: &str) -> u32 {
    get_first_digit(input).expect("There should be a first digit") * 10
        + get_last_digit(input).expect("There should be a last digit")
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input_filepath = args.get(1).expect("Please specify the input filepath");
    let input_file = File::open(input_filepath)?;

    let input_file_buf = BufReader::new(input_file);

    let mut total = 0 as u32;
    for line in input_file_buf
        .lines()
        .map(|line| line.expect("Expecting a line here"))
    {
        let number = get_number(&line);
        total += number;
    }

    println!("{}", total);

    Ok(())
}
