use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

type DynResult<T> = Result<T, Box<dyn Error>>;

fn split_two<'a>(x: &'a str, sep: char) -> DynResult<[&'a str; 2]> {
    let values: Vec<&str> = x.split(sep).collect();

    if values.len() != 2 {
        return Err(Box::from(format!("Mismatched split count for {:?} splitting on {:?}", x, sep)));
    }

    Ok([values[0], values[1]])
}

fn main() -> DynResult<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    let mut total_points: u64 = 0;
    let mut copy_count = [0 as i32; 256];
    for liner in reader.lines() {
        let line = liner?;

        let [card_num_text, nums] = split_two(&line, ':')?;
        let card_num = card_num_text[4..].trim().parse::<usize>()?;

        let [winner_r, yours_r] = split_two(nums, '|')?
            .map(|text| text
                .split(' ')
                .filter(|x| x.trim() != "")
                .map(|x| x.parse().map_err(|e: ParseIntError| e.into()))
                .collect::<DynResult<Vec<i32>>>()
            );


        let winner = winner_r?;
        let yours = yours_r?;
        let win_count = yours
            .iter()
            .filter(|x| winner.contains(x))
            .count();

        println!("winner = {:?}, yours = {:?}, count = {}", winner, yours, win_count);
        total_points += (1 << win_count) >> 1;

        copy_count[card_num] += 1;
        for i in card_num + 1 .. card_num + win_count + 1 {
            copy_count[i] += copy_count[card_num];
        }
    }

    println!("total points = {}, card count = {}", total_points, copy_count.iter().sum::<i32>());

    Ok(())
}
