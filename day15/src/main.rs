use std::fs;
use std::error::Error;

type DynResult<T> = Result<T, Box<dyn Error>>;

fn calculate_hash(x: &str) -> i32 {
    x
        .chars()
        .fold(0, |curr, c| ((curr + (c as i32)) * 17) % 256)
}

fn solve_part1() -> DynResult<()> {
    let content = fs::read_to_string("./input.txt")?;

    let result: i32 = content
        .trim()
        .split(',')
        .map(|x| { println!("{}", x); x })
        .map(calculate_hash)
        .map(|x| { println!("{}", x); x })
        .sum();

    println!("result = {}", result);

    Ok(())
}

fn solve_part2() -> DynResult<()> {
    let content = fs::read_to_string("./input.txt")?;
    let mut boxes: Vec<Vec<(String, i32)>> = Vec::with_capacity(256);

    for _ in 0..256 {
        boxes.push(vec![]);
    }

    for cmd in content.trim().split(',') {
        let last = cmd.chars().nth_back(0).ok_or("Empty command")?;
        let label = cmd
            .chars()
            .take_while(|x| x.is_alphabetic())
            .collect::<String>();
        let box_num = calculate_hash(label.as_str()) as usize;

        let label_in_box = boxes[box_num]
            .iter()
            .position(|(label2, _)| label == *label2);

        if last == '-' {
            if let Some(i) = label_in_box {
                boxes[box_num].remove(i);
            }
        }
        else {
            let num = last.to_digit(10).ok_or("Invalid num")? as i32;
            match label_in_box {
                Some(i) => boxes[box_num][i] = (label, num),
                None => boxes[box_num].push((label, num)),
            };
        }

        //println!("Boxes after cmd {}: {:?}", cmd, boxes);
    }

    let mut sum = 0;
    for i in 0 .. 256 {
        for (j, (_, num)) in boxes[i].iter().enumerate() {
            sum += (i + 1) * (j + 1) * (*num as usize);
        }
    }

    println!("focusing power = {}", sum);

    Ok(())
}

fn main() -> DynResult<()> {
    solve_part1()?;
    solve_part2()?;
    Ok(())
}
