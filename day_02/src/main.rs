use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Debug)]
struct SetCubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl SetCubes {
    fn from_raw_set(raw_set: &str) -> Self {
        let cubes = raw_set.split(", ").collect::<Vec<&str>>();

        let mut color_cubes = HashMap::new();

        for cube in cubes {
            // "1 blue"
            let pieces = cube.split(' ').collect::<Vec<&str>>();

            let amount = pieces[0]
                .parse::<u32>()
                .expect("Expected cubes amount here");
            let color = pieces[1];

            if let Some(prev_amount) = color_cubes.insert(color, amount) {
                let amount = color_cubes.get_mut(color).expect("Expecting an entry here");
                *amount += prev_amount;
            }
        }

        Self {
            red: *color_cubes.get("red").unwrap_or(&0),
            green: *color_cubes.get("green").unwrap_or(&0),
            blue: *color_cubes.get("blue").unwrap_or(&0),
        }
    }

    fn amount(&self, color: &str) -> u32 {
        match color {
            "red" => self.red,
            "green" => self.green,
            "blue" => self.blue,
            _ => 0,
        }
    }

    fn possible(&self, available_cubes: &HashMap<&str, u32>) -> bool {
        for color in ["red", "green", "blue"] {
            let max = *available_cubes
                .get(color)
                .expect("There should be am entry for this color");
            if max < self.amount(color) {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
struct GameCubes {
    id: u32,
    sets: Vec<SetCubes>,
}

impl GameCubes {
    fn from_line(line: &str) -> Self {
        let pieces = line.split(": ").collect::<Vec<&str>>();
        let id = pieces[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .expect("Expected game number here");

        let raw_sets = pieces[1].split("; ").collect::<Vec<&str>>();

        let mut sets = Vec::with_capacity(raw_sets.len());
        for raw_set in raw_sets {
            sets.push(SetCubes::from_raw_set(raw_set));
        }

        Self { id, sets }
    }

    fn possible(&self, available_cubes: &HashMap<&str, u32>) -> bool {
        for set in &self.sets {
            if !set.possible(available_cubes) {
                return false;
            }
        }

        true
    }

    fn max_cubes(&self) -> u32 {
        let mut max_red_cubes = 0;
        let mut max_green_cubes = 0;
        let mut max_blue_cubes = 0;
        for set in &self.sets {
            let red_cubes = set.amount("red");
            let green_cubes = set.amount("green");
            let blue_cubes = set.amount("blue");

            if red_cubes > max_red_cubes {
                max_red_cubes = red_cubes;
            }

            if green_cubes > max_green_cubes {
                max_green_cubes = green_cubes;
            }

            if blue_cubes > max_blue_cubes {
                max_blue_cubes = blue_cubes;
            }
        }

        max_red_cubes * max_green_cubes * max_blue_cubes
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input_filepath = args.get(1).expect("Please specify the input filepath");
    let input_file = File::open(input_filepath)?;
    let input_file_buf = BufReader::new(input_file);

    let available_cubes: HashMap<&str, u32> =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut possible_games_id_sum = 0;
    let mut minimum_games_cubes = 0;
    for line in input_file_buf
        .lines()
        .map(|line| line.expect("Expecting a line here"))
    {
        let game_cube = GameCubes::from_line(&line);
        if game_cube.possible(&available_cubes) {
            possible_games_id_sum += game_cube.id;
        }
        minimum_games_cubes += game_cube.max_cubes();
    }

    println!("{}", possible_games_id_sum);
    println!("{}", minimum_games_cubes);

    Ok(())
}
