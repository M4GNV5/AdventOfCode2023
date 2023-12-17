use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

type DynResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Transformation {
    destination: i64,
    source: i64,
    len: i64,
}

fn transform_numbers(nums: &mut Vec<i64>, mappings: &Vec<Transformation>) {
    for i in 0..nums.len() {
        let mut found = false;
        for mapping in mappings {
            if nums[i] >= mapping.source && nums[i] < mapping.source + mapping.len {
                //println!("    {} -> {}", nums[i], mapping.destination + nums[i] - mapping.source);
                nums[i] = mapping.destination + nums[i] - mapping.source;
                found = true;
                break;
            }
        }

        if !found {
            //println!("    keep {}", nums[i]);
        }
    }
}

fn solve_part1() -> DynResult<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    let mut first_mapping = true;
    let mut current: Vec<i64> = vec![];
    let mut mappings: Vec<Transformation> = vec![];

    for liner in reader.lines() {
        let line = liner?;
        let split = line.split(" ").collect::<Vec<&str>>();
        if split.len() == 1 && split[0].len() == 0 {
            // ignore empty lines
        }
        else if split.len() < 2 {
            panic!("Unknown/malformed line: {:?}", split);
        }
        else if split[0] == "seeds:" {
            current.extend(split
                .iter()
                .skip(1)
                .map(|x| x.parse().map_err(|e: ParseIntError| e.into()))
                .collect::<DynResult<Vec<i64>>>()?
                .iter()
            );
        }
        else if split[1] == "map:" {
            if !first_mapping {
                //println!("{}:", split[0]);
                transform_numbers(&mut current, &mappings);
                mappings.clear();
            }
            first_mapping = false;
        }
        else {
            mappings.push(Transformation {
                destination: split[0].parse()?,
                source: split[1].parse()?,
                len: split[2].parse()?,
            });
        }
    }

    transform_numbers(&mut current, &mappings);

    println!("min location: {:?}", current.iter().min());

    Ok(())
}



// since the solution to part 1 would go OOM with part 2 we need a different approach
// this approach for part 2 goes for all locations backwards through the transformations
// until a seed matching to one of the locations is found
fn transform_number_reverse(num: i64, mappings: &Vec<Transformation>) -> i64 {
    let mut found = false;
    for mapping in mappings {
        if num >= mapping.destination && num < mapping.destination + mapping.len {
            //println!("    {} <- {}", num, num + mapping.source - mapping.destination);
            return num + mapping.source - mapping.destination;
        }
    }

    //println!("    keep {}", num);
    num
}

fn solve_part2() -> DynResult<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    let mut seeds: Vec<(i64, i64)> = vec![];
    let mut mappings: Vec<Vec<Transformation>> = vec![];
    let mut current_mappings: Vec<Transformation> = vec![];

    for liner in reader.lines() {
        let line = liner?;
        let split = line.split(" ").collect::<Vec<&str>>();
        if split.len() == 1 && split[0].len() == 0 {
            // ignore empty lines
        }
        else if split.len() < 2 {
            panic!("Unknown/malformed line: {:?}", split);
        }
        else if split[0] == "seeds:" {
            seeds.extend(split
                .iter()
                .skip(1)
                .map(|x| x.parse().map_err(|e: ParseIntError| e.into()))
                .collect::<DynResult<Vec<i64>>>()?
                .as_slice()
                .chunks_exact(2)
                .map(|x| (x[0], x[1]))
            );
        }
        else if split[1] == "map:" {
            if current_mappings.len() > 0 {
                mappings.push(current_mappings);
                current_mappings = vec![];
            }
        }
        else {
            current_mappings.push(Transformation {
                destination: split[0].parse()?,
                source: split[1].parse()?,
                len: split[2].parse()?,
            });
        }
    }

    mappings.reverse();

    // XXX we just use a very large maximum location here
    for i in 0..999999999999 {
        if i % 100000 == 0 {
            println!("Trying location {}", i);
        }
        let mut curr = i;
        for mapping in &mappings {
            curr = transform_number_reverse(curr, mapping);
        }

        for (start, len) in &seeds {
            if curr >= *start && curr < *start + *len {
                println!("min location: {} -> ... -> {}", curr, i);
                return Ok(());
            }
        }
    }

    Err("No minimal location found :(".into())
}

fn main() -> DynResult<()> {
    solve_part1()?;
    solve_part2()?;
    Ok(())
}