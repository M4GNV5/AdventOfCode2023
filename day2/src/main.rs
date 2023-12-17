use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::max;

type DynResult<T> = Result<T, Box<dyn Error>>;

struct GameRound {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameRound {
    fn empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn combine(a: GameRound, b: &GameRound) -> GameRound {
        GameRound {
            red: a.red + b.red,
            green: a.green + b.green,
            blue: a.blue + b.blue,
        }
    }

    fn max(a: GameRound, b: GameRound) -> GameRound {
        GameRound {
            red: max(a.red, b.red),
            green: max(a.green, b.green),
            blue: max(a.blue, b.blue),
        }
    }
}

fn split_two<'a>(x: &'a str, sep: char) -> Result<[&'a str; 2], Box<dyn Error>> {
    let values: Vec<&str> = x.split(sep).collect();

    if values.len() != 2 {
        return Err(Box::from(format!("Mismatched split count for {:?} splitting on {:?}", x, sep)));
    }

    Ok([values[0], values[1]])
}

fn main() -> DynResult<()> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut id_sum_12_13_14: usize = 0;
    let mut max_cubes_power_sum: usize = 0;
    for liner in reader.lines() {
        let line = liner?;
        let [prefix, rounds_text] = split_two(line.as_str(), ':')?;
        let [_, num_text] = split_two(prefix, ' ')?;
        let num = num_text.parse::<usize>()?;

        let rounds = rounds_text
            .split(';')
            .map(|round| {
                let picks: Vec<GameRound> = round
                    .split(',')
                    .map(|take| {
                        let [amount_text, color] = split_two(take.trim(), ' ')?;
                        let amount = amount_text.parse()?;
                        Ok(GameRound {
                            red: if color == "red" { amount } else { 0 },
                            green: if color == "green" { amount } else { 0 },
                            blue: if color == "blue" { amount } else { 0 },
                        })
                    })
                    .collect::<Result<Vec<GameRound>, Box<dyn Error>>>()?;

                Ok(picks.iter().fold(GameRound::empty(), GameRound::combine))
            })
            .collect::<DynResult<Vec<GameRound>>>()?;

        let possible_with_12_13_14 = rounds
            .iter()
            .all(|x| x.red <= 12 && x.green <= 13 && x.blue <= 14);

        if possible_with_12_13_14 {
            id_sum_12_13_14 += num;
        }

        let mut max_gameround = GameRound::empty();
        for round in rounds {
            max_gameround = GameRound::max(max_gameround, round);
        }
        max_cubes_power_sum += max_gameround.red * max_gameround.green * max_gameround.blue;
    }

    println!("day2: part1 = {}, part2 = {:?}", id_sum_12_13_14, max_cubes_power_sum);

    Ok(())
}
