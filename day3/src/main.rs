use std::error::Error;
use std::fs;
use std::str::from_utf8;
use itertools::Itertools;

type DynResult<T> = Result<T, Box<dyn Error>>;

const OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    //(0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];


fn get_char(lines: &Vec<&[u8]>, x: i32, y: i32) -> Option<u8> {
    if x < 0 || y < 0 || y as usize >= lines.len() || x as usize >= lines[y as usize].len() {
        None
    }
    else {
        Some(lines[y as usize][x as usize])
    }
}
fn get_adjacent_chars(lines: &Vec<&[u8]>, x: i32, y: i32) -> Vec<(i32, i32, u8)> {
    OFFSETS
        .iter()
        .map(|(dx, dy)| get_char(lines, x + dx, y + dy)
            .map(|c| (x + dx, y + dy, c))
        )
        .flatten()
        .collect::<Vec<_>>()
}

fn main() -> DynResult<()> {
    let content = fs::read_to_string("input.txt")?;
    let lines: Vec<&[u8]> = content
        .lines()
        .map(|x| x.as_bytes())
        .collect();

    // part 1
    let mut part_number_sum = 0;
    for y in 0 .. lines.len() {
        let mut is_part_number = false;
        let mut num = 0;
        for x in 0 .. lines[y].len() + 1 {
            if x < lines[y].len() && lines[y][x].is_ascii_digit() {
                num *= 10;
                num += (lines[y][x] - '0' as u8) as u64;

                if !is_part_number {
                    is_part_number = get_adjacent_chars(&lines, x as i32, y as i32)
                        .iter()
                        .any(|(_, _, c)| !c.is_ascii_digit() && *c != '.' as u8);
                }
            }
            else {
                if is_part_number {
                    println!("found part number {} end x/y = {}/{}", num, x, y);
                    part_number_sum += num;
                    is_part_number = false;
                } else if num != 0 {
                    println!("non part number {} end x/y = {}/{}", num, x, y);
                }
                num = 0;
            }
        }
    }
    println!("part number sum = {}", part_number_sum);

    // part 2
    let mut gear_ratio_sum: i64 = 0;
    for y in 0 .. lines.len() {
        for x in 0 .. lines[y].len() {
            if lines[y][x] == '*' as u8 {
                let adjacent_nums = get_adjacent_chars(&lines, x as i32, y as i32)
                    .iter()
                    .filter(|(_, _, c)| c.is_ascii_digit())
                    .map(|(x, y, c)| {
                        let xu = *x as usize;
                        let yu = *y as usize;
                        let firstX = lines[yu][0 .. xu]
                            .iter()
                            .rev()
                            .position(|k| !k.is_ascii_digit())
                            .map(|k| xu - k)
                            .unwrap_or(0);
                        let mut lastX = lines[yu][xu .. ]
                            .iter()
                            .position(|k| !k.is_ascii_digit())
                            .map(|k| xu + k)
                            .unwrap_or(lines[yu].len());

                        println!("xu = {}, yu = {}, firstX = {}, lastX = {}, range1 = {}, range2 = {}, result range = {}",
                            xu, yu,
                            firstX, lastX,
                            from_utf8(&lines[yu][0 .. xu]).unwrap(),
                            from_utf8(&lines[yu][xu .. ]).unwrap(),
                            from_utf8(&lines[yu][firstX .. lastX]).unwrap()
                        );
                        str::parse::<i64>(from_utf8(&lines[yu][firstX .. lastX]).unwrap()).unwrap()
                    })
                    .into_iter()
                    .unique()
                    .collect::<Vec<_>>();

                if adjacent_nums.len() == 2 {
                    gear_ratio_sum += adjacent_nums[0] * adjacent_nums[1];
                }
            }
        }
    }

    println!("gear ratio sum: {}", gear_ratio_sum);


    Ok(())
}
