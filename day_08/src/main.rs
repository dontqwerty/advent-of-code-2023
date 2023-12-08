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

    let mut directions = vec![];
    let mut map = HashMap::new();
    for (i, line) in input_file_buf
        .lines()
        .enumerate()
        .map(|(i, line)| (i, line.expect("Expecting a line here")))
    {
        if i == 0 {
            directions = line.chars().collect();
            continue;
        }

        if i == 1 {
            continue;
        }

        let line = line
            .to_owned()
            .split(" = ")
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        dbg!(line.clone());

        let part_0 = line[0].clone();
        let part_1 = &line[1].chars().collect::<Vec<char>>();
        let part_1 = part_1[1..line[1].len() - 1]
            .iter()
            .filter(|c| **c != ',' && **c != ' ')
            .map(|c| c.clone())
            .collect::<Vec<char>>();

        let mut p1 = String::new();
        let mut p2 = String::new();
        for (i, p) in part_1.into_iter().enumerate() {
            if i <= 2 {
                p1.push(p);
            } else {
                p2.push(p);
            }
        }

        map.insert(part_0, (p1, p2));
    }

    let mut done = false;
    let mut steps = 0;
    let mut start = map.get(&"AAA".to_string()).unwrap();
    while done == false {
        for direction in directions.clone() {
            steps += 1;
            let dest = if direction == 'L' { &start.0 } else { &start.1 };
            if dest == &"ZZZ".to_string() {
                done = true;
                break;
            }
            start = map.get(dest).unwrap();
            dbg!(start);
            dbg!(dest);
        }
    }

    dbg!(map);
    dbg!(steps);

    Ok(())
}
