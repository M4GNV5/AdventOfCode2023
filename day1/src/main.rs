use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const words: [&'static str; 10] = [
    "XXXX zero not included XXXX",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn parse_digit(line: &str, i: usize, use_words: bool) -> Option<u32> {
    let char_num = line
        .chars()
        .nth(i)
        .and_then(|x| x.to_digit(10));

    if char_num.is_some() {
        char_num
    }
    else if use_words {
        for (num, word) in words.iter().enumerate() {
            if i + word.len() <= line.len() && line[i .. i + word.len()] == **word {
                return Some(num as u32);
            }
        }

        None
    }
    else {
        None
    }
}

fn solve_day1(use_words: bool) -> Result<u32, Box<dyn Error>> {
    let reader = BufReader::new(File::open("input.txt")?);
    let mut sum = 0;
    for liner in reader.lines() {
        let line = liner?;
        let parsed: Vec<Option<u32>> = (0..line.len())
            .into_iter()
            .map(|i| parse_digit(&line, i, use_words))
            .collect();
        let first = parsed
            .iter()
            .find(|x| x.is_some())
            .and_then(|x| *x)
            .ok_or("no first digit found")?;
        let last = parsed
            .iter()
            .rfind(|x| x.is_some())
            .and_then(|x| *x)
            .ok_or("no first digit found")?;

        sum += first * 10 + last;
    }

    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let part1 = 0;//solve_day1(false)?;
    let part2 = solve_day1(true)?;
    println!("sum: part1 = {}, part2 = {}", part1, part2);

    Ok(())
}
