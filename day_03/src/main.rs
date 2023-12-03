use std::{
    collections::HashMap,
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

impl DigitPosition {
    fn new(index: usize, number_len: usize) -> Self {
        if number_len == 1 {
            DigitPosition::StartEnd
        } else if index == 0 {
            DigitPosition::Start
        } else if index == number_len - 1 {
            DigitPosition::End
        } else {
            DigitPosition::Middle
        }
    }
}

#[derive(Clone, Debug)]
struct Digit {
    value: u32,
    row_ix: usize,
    col_ix: usize,
}

impl Digit {
    const INDEX_SHIFTS_FOR_START: &[(i32, i32)] = &[(-1, -1), (-1, 0), (0, -1), (1, -1), (1, 0)];
    const INDEX_SHIFT_FOR_MIDDLE: &[(i32, i32)] = &[(-1, 0), (1, 0)];
    const INDEX_SHIFT_FOR_END: &[(i32, i32)] = &[(-1, 0), (-1, 1), (0, 1), (1, 0), (1, 1)];
    const INDEX_SHIFT_FOR_START_END: &[(i32, i32)] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    fn get_adjacent_gears(
        &self,
        pos: DigitPosition,
        lines: &[Vec<char>],
    ) -> Option<Vec<(usize, usize)>> {
        let mut ret = None;

        let index_shifts = match pos {
            DigitPosition::Start => Self::INDEX_SHIFTS_FOR_START,
            DigitPosition::Middle => Self::INDEX_SHIFT_FOR_MIDDLE,
            DigitPosition::End => Self::INDEX_SHIFT_FOR_END,
            DigitPosition::StartEnd => Self::INDEX_SHIFT_FOR_START_END,
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

            let adjacent = &lines[adjacent_row_ix][adjacent_col_ix];

            if !(adjacent != &'.' && adjacent.is_ascii_punctuation()) {
                continue;
            }

            if ret.is_none() {
                ret = Some(vec![])
            }

            if adjacent == &'*' {
                let mut ret_value = ret.expect("Return value should be some by now");
                ret_value.push((adjacent_row_ix, adjacent_col_ix));
                ret = Some(ret_value);
            }
        }

        ret
    }
}

#[derive(Clone, Debug)]
struct Number {
    digits: Vec<Digit>,
}

impl Number {
    fn as_number(&self) -> u32 {
        let mut number = 0;
        for (ix, digit) in self.digits.iter().rev().enumerate() {
            number += digit.value * (10_u32).pow(ix as u32);
        }
        number
    }

    fn as_part_number(
        &self,
        lines: &[Vec<char>],
        possible_gears: &mut HashMap<(usize, usize), Vec<u32>>,
    ) -> Option<u32> {
        let mut ret = None;
        for (digit_ix, digit) in self.digits.iter().enumerate() {
            let literal_digit_pos = DigitPosition::new(digit_ix, self.digits.len());

            if let Some(gears) = digit.get_adjacent_gears(literal_digit_pos, lines) {
                let part_number = self.as_number();
                if ret.is_none() {
                    ret = Some(part_number);
                }

                for gear in gears {
                    if let Some(prev_gear_part_numbers) =
                        possible_gears.insert(gear, vec![part_number])
                    {
                        let gear_part_numbers = possible_gears
                            .get_mut(&gear)
                            .expect("This entry should exist");
                        gear_part_numbers.extend(prev_gear_part_numbers);
                    }
                }
            }
        }

        ret
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

    let mut missing_part_number = 0;
    let mut possible_gears = HashMap::new();
    for (row_ix, line) in lines.iter().enumerate() {
        let mut number = Number { digits: vec![] };
        for (col_ix, char) in line.iter().enumerate() {
            if char.is_numeric() {
                let digit = Digit {
                    value: char.to_digit(10).expect("This char should be a digit"),
                    row_ix,
                    col_ix,
                };
                number.digits.push(digit);
            } else if !number.digits.is_empty() {
                if let Some(part_number) =
                    number.as_part_number(lines.as_slice(), &mut possible_gears)
                {
                    missing_part_number += part_number;
                }
                number.digits.truncate(0);
            }
        }
    }

    println!("{}", missing_part_number);

    let gear_ratio: u32 = possible_gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum();

    println!("{}", gear_ratio);

    Ok(())
}
