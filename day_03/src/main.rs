use std::{
    cmp, env,
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Clone, Debug)]
struct LiteralDigit {
    digit: u32,
    row_ix: usize,
    col_ix: usize,
}

impl LiteralDigit {
    fn possible(&self, lines: &[Vec<char>]) -> bool {
        println!("{:?}", self);
        for row_ix_shift in -1..2 as i32 {
            for col_ix_shift in -1..2 as i32 {
                // println!("shifts: {}, {}", row_ix_shift, col_ix_shift);

                if row_ix_shift == 0 && col_ix_shift == 0 {
                    // println!("Zero shift skip");
                    continue;
                }

                let adjacent_row_ix = cmp::max(0, self.row_ix as i32 + row_ix_shift) as usize;
                let adjacent_col_ix = cmp::max(0, self.col_ix as i32 + col_ix_shift) as usize;

                if adjacent_row_ix == self.row_ix && adjacent_col_ix == self.col_ix {
                    // println!("Self skip");
                    continue;
                }
                // println!("adjacent: {}, {}", adjacent_row_ix, adjacent_col_ix);

                if let Some(Some(adjacent)) = lines
                    .get(adjacent_row_ix)
                    .map(|row| row.get(adjacent_col_ix))
                {
                    println!("adjacent value: {}", adjacent);
                    if adjacent != &'.' && adjacent.is_ascii_punctuation() {
                        println!("-------------------------> true");
                        return true;
                    }
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
            number += literal_digit.digit * (10 as u32).pow(ix as u32);
        }
        number
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

    let mut part_number = 0;
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
                println!("####################################################");
                println!("{:?}", literal_number.as_number());
                for literal_digit in &literal_number.digits {
                    if literal_digit.possible(lines.as_slice()) {
                        part_number += literal_number.as_number();
                        break;
                    }
                }
                literal_number.empty();
            }
        }
    }

    println!("{}", part_number);

    Ok(())
}
