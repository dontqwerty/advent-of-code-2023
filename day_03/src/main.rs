use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Error},
};

enum DigitPosition {
    Start,
    Middle,
    End,
    StartEnd,
}

#[derive(Clone, Debug)]
struct LiteralDigit {
    digit: u32,
    row_ix: usize,
    col_ix: usize,
}

impl LiteralDigit {
    fn is_digit_of_part_number(&self, pos: DigitPosition, lines: &[Vec<char>]) -> bool {
        let index_shifts = match pos {
            DigitPosition::Start => vec![(-1, -1), (-1, 0), (0, -1), (1, -1), (1, 0)],
            DigitPosition::Middle => vec![(-1, 0), (1, 0)],
            DigitPosition::End => vec![(-1, 0), (-1, 1), (0, 1), (1, 0), (1, 1)],
            DigitPosition::StartEnd => vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ],
        };

        for (row_shift, col_shift) in index_shifts {
            let adjacent_row_ix = self.row_ix as i32 + row_shift;
            if adjacent_row_ix < 0 || adjacent_row_ix > (lines.len() as i32 - 1) {
                continue;
            }
            let adjacent_row_ix = adjacent_row_ix as usize;

            let adjacent_col_ix = self.col_ix as i32 + col_shift;
            if adjacent_col_ix < 0 || adjacent_col_ix > (lines[adjacent_row_ix].len() as i32 - 1) {
                continue;
            }
            let adjacent_col_ix = adjacent_col_ix as usize;

            if adjacent_row_ix == self.row_ix && adjacent_col_ix == self.col_ix {
                continue;
            }

            if let Some(Some(adjacent)) = lines
                .get(adjacent_row_ix)
                .map(|row| row.get(adjacent_col_ix))
            {
                if adjacent != &'.' && adjacent.is_ascii_punctuation() {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Clone, Debug)]
struct LiteralNumber {
    digits: Vec<LiteralDigit>,
}

impl LiteralNumber {
    fn push_digit(&mut self, digit: LiteralDigit) {
        self.digits.push(digit)
    }

    fn is_empty(&self) -> bool {
        self.digits.is_empty()
    }

    fn empty(&mut self) {
        self.digits.truncate(0)
    }

    fn as_number(&self) -> u32 {
        let mut number = 0;
        for (ix, literal_digit) in self.digits.iter().rev().enumerate() {
            number += literal_digit.digit * (10_u32).pow(ix as u32);
        }
        number
    }

    fn as_part_number(&self, lines: &[Vec<char>]) -> Option<u32> {
        for (literal_digit_ix, literal_digit) in self.digits.iter().enumerate() {
            let literal_digit_pos = if self.digits.len() == 1 {
                DigitPosition::StartEnd
            } else if literal_digit_ix == 0 {
                DigitPosition::Start
            } else if literal_digit_ix == self.digits.len() - 1 {
                DigitPosition::End
            } else {
                DigitPosition::Middle
            };

            if literal_digit.is_digit_of_part_number(literal_digit_pos, lines) {
                return Some(self.as_number());
            }
        }

        None
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input_filepath = args.get(1).expect("Please specify the input filepath");
    let input_file = File::open(input_filepath)?;
    let input_file_buf = BufReader::new(input_file);

    let mut lines = vec![];

    for line in input_file_buf
        .lines()
        .map(|line| line.expect("Expecting a line here"))
    {
        let mut line = line.chars().collect::<Vec<char>>();
        line.push('.');
        lines.push(line);
    }

    let mut part_number_total = 0;
    for (row_ix, line) in lines.iter().enumerate() {
        let mut literal_number = LiteralNumber { digits: vec![] };
        for (col_ix, char) in line.iter().enumerate() {
            if char.is_numeric() {
                let digit = char.to_digit(10).unwrap();
                literal_number.push_digit(LiteralDigit {
                    digit,
                    row_ix,
                    col_ix,
                });
            } else if !literal_number.is_empty() {
                if let Some(part_number) = literal_number.as_part_number(lines.as_slice()) {
                    part_number_total += part_number;
                }
                literal_number.empty();
            }
        }
    }

    println!("{}", part_number_total);

    Ok(())
}
