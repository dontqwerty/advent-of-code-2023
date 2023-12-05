use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Error},
    thread,
};

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
        lines.push(line);
    }

    let mut seeds = vec![];
    let mut map = vec![];
    let mut maps = vec![];
    let mut fetch_map = false;
    for (line_ix, line) in lines.iter().enumerate() {
        if line_ix == 0 {
            seeds = line.split(':').collect::<Vec<&str>>()[1]
                .split_ascii_whitespace()
                .map(|number| number.parse::<i64>().expect("Expecting a parsable seed"))
                .collect::<Vec<i64>>();
            continue;
        }

        if line.contains("map:") {
            fetch_map = true;
            continue;
        }

        if fetch_map {
            if line.is_empty() {
                fetch_map = false;
            } else {
                let numbers = line
                    .split_ascii_whitespace()
                    .map(|number| number.parse::<i64>().expect("Expecting a parsable number"))
                    .collect::<Vec<i64>>();
                map.push(vec![numbers[1], numbers[2], numbers[0] - numbers[1]]);
            }
        }

        if (!fetch_map || line_ix == lines.len() - 1) && !map.is_empty() {
            maps.push(map.to_owned());
            map.truncate(0);
        }
    }

    let mut best_seed = i64::MAX;
    for mut seed in seeds.clone() {
        for map in &maps {
            let mut dest = None;
            for map_line in map {
                if seed >= map_line[0] && seed < map_line[0] + map_line[1] {
                    dest = Some(seed + map_line[2]);
                    break;
                }
            }
            if let Some(dest) = dest {
                seed = dest;
            }
        }
        if seed < best_seed {
            best_seed = seed;
        }
    }

    println!("{}", best_seed);

    let mut handlers = Vec::with_capacity(seeds.len() / 2);
    for seed_range in seeds.chunks(2) {
        let handler = thread::spawn({
            let maps = maps.clone();
            let seed_range = seed_range.to_vec();
            move || {
                let mut best_seed = i64::MAX;
                for mut seed in seed_range[0]..seed_range[0] + seed_range[1] {
                    for map in &maps {
                        let mut dest = None;
                        for map_line in map {
                            if seed >= map_line[0] && seed < map_line[0] + map_line[1] {
                                dest = Some(seed + map_line[2]);
                                break;
                            }
                        }
                        if let Some(dest) = dest {
                            seed = dest;
                        }
                    }
                    if seed < best_seed {
                        best_seed = seed;
                    }
                }

                best_seed
            }
        });

        handlers.push(handler);
    }

    let mut best_seed = i64::MAX;
    for handler in handlers {
        let seed = handler.join().unwrap();
        if seed < best_seed {
            best_seed = seed;
        }
    }

    println!("{}", best_seed);

    Ok(())
}
