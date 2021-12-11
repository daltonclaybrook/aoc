use std::path::Path;
use std::fs;

#[derive(Debug)]
pub enum Day3Error {
    FileError,
    NumParseError,
    NoResult
}

/// Day 3, Part 1
pub fn determine_power_consumption() -> Result<usize, Day3Error> {
    let lines = read_input_lines()?;
    let counts_per_index = lines
        .iter()
        .fold(vec![], |counts: Vec<usize>, string| {
            let mut counts = counts;
            string.char_indices().for_each(|idx_and_char| {
                if counts.len() <= idx_and_char.0 {
                    counts.insert(idx_and_char.0, 0);
                }
                if idx_and_char.1 == '1' {
                    counts[idx_and_char.0] += 1;
                }
            });
            counts
        });
    
    let count_of_nums = lines.len();
    let gamma_str = counts_per_index.iter().fold(String::from(""), |result, count| {
        if *count > count_of_nums / 2 {
            result + "1"
        } else {
            result + "0"
        }
    });
    let epsilon_str = gamma_str.chars()
        .map(|char| if char == '1' { '0' } else { '1' })
        .collect::<String>();

    let gamma = usize::from_str_radix(&gamma_str, 2).map_err(|_| Day3Error::NumParseError)?;
    let epsilon = usize::from_str_radix(&epsilon_str, 2).map_err(|_| Day3Error::NumParseError)?;

    Ok(gamma * epsilon)
}

/// Day 3, part 2
pub fn determine_life_support_rating() -> Result<usize, Day3Error> {
    let lines = read_input_lines()?;
    let ox_rating = determine_ox_or_co2_rating(lines.clone(), true)?;
    let co2_rating = determine_ox_or_co2_rating(lines, false)?;

    Ok(ox_rating * co2_rating)
}

// MARK: - Private helpers

fn determine_ox_or_co2_rating(lines: Vec<String>, prefers_ones: bool) -> Result<usize, Day3Error> {
    let mut lines = lines;
    let str_length = lines.first().unwrap().len();
    for char_idx in 0..str_length {
        let mut ones: Vec<String> = vec![];
        let mut zeroes: Vec<String> = vec![];

        for line in lines.into_iter() {
            let char = line.chars().nth(char_idx).unwrap();
            if char == '1' {
                ones.insert(ones.len(), line);
            } else {
                zeroes.insert(zeroes.len(), line);
            }
        }

        if (prefers_ones && ones.len() >= zeroes.len()) || (!prefers_ones && zeroes.len() > ones.len()) {
            lines = ones
        } else {
            lines = zeroes
        }

        if lines.len() == 1 {
            return usize::from_str_radix(lines.first().unwrap(), 2)
                .map_err(|_| Day3Error::NumParseError);
        }
    }

    Err(Day3Error::NoResult)
}

fn read_input_lines() -> Result<Vec<String>, Day3Error> {
    let path = Path::new("./resources/day3.txt");
    let contents = fs::read_to_string(path).map_err(|_| Day3Error::FileError)?;
    let lines = contents.split("\n").map(|s| s.to_string()).collect::<Vec<String>>();
    Ok(lines)
}