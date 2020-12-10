use advent_of_code_2020::UnsolvedError;
use regex::Regex;
use std::error::Error;
use std::str::{FromStr, Lines};

#[derive(Debug, PartialEq)]
struct Passport {
    birth_year: u32,
    issue_year: u32,
    expiration_year: u32,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<String>,
}

fn find_passport_data(data: &Vec<&str>, prefix: &str) -> Result<String, UnsolvedError> {
    data.iter()
        .flat_map(|s| s.strip_prefix(prefix))
        .map(|s| String::from(s))
        .next()
        .ok_or(UnsolvedError)
}

fn validate_year_data(
    data: &Vec<&str>,
    prefix: &str,
    min: u32,
    max: u32,
) -> Result<u32, UnsolvedError> {
    let year_as_str = find_passport_data(&data, prefix)?;
    if year_as_str.len() != 4 {
        return Err(UnsolvedError);
    }
    let year = year_as_str.parse::<u32>().unwrap();
    if year < min || year > max {
        return Err(UnsolvedError);
    }
    Ok(year)
}

fn validate_height(data: &Vec<&str>) -> Result<String, UnsolvedError> {
    let height = find_passport_data(&data, "hgt:")?;
    // a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    if height
        .strip_suffix("cm")
        .and_then(|h| h.parse::<u32>().ok())
        .filter(|h| *h >= 150 && *h <= 193)
        .is_some()
    {
        return Ok(height);
    }

    if height
        .strip_suffix("in")
        .and_then(|h| h.parse::<u32>().ok())
        .filter(|h| *h >= 59 && *h <= 76)
        .is_some()
    {
        return Ok(height);
    }

    Err(UnsolvedError)
}

fn validate_hair_color(data: &Vec<&str>) -> Result<String, UnsolvedError> {
    let hair_color = find_passport_data(&data, "hcl:")?;
    // a # followed by exactly six characters 0-9 or a-f.
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    if !re.is_match(hair_color.as_str()) {
        return Err(UnsolvedError);
    }
    Ok(hair_color)
}

fn validate_eye_color(data: &Vec<&str>) -> Result<String, UnsolvedError> {
    let eye_color = find_passport_data(&data, "ecl:")?;
    // exactly one of: amb blu brn gry grn hzl oth.
    let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    if !re.is_match(eye_color.as_str()) {
        return Err(UnsolvedError);
    }
    Ok(eye_color)
}

fn validate_passport_id(data: &Vec<&str>) -> Result<String, UnsolvedError> {
    let passport_id = find_passport_data(&data, "pid:")?;
    let re = Regex::new(r"^\d{9}$").unwrap();
    if !re.is_match(passport_id.as_str()) {
        return Err(UnsolvedError);
    }
    Ok(passport_id)
}

impl FromStr for Passport {
    type Err = UnsolvedError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = input.split_ascii_whitespace().collect();
        return Ok(Passport {
            // birth year: 4 digits &  at least 1920 & at most 2002
            birth_year: validate_year_data(&data, "byr:", 1920, 2002)?,
            // issue year: four digits; at least 2010 and at most 2020.
            issue_year: validate_year_data(&data, "iyr:", 2010, 2020)?,
            // expiration year: four digits; at least 2020 and at most 2030.
            expiration_year: validate_year_data(&data, "eyr:", 2020, 2030)?,
            height: validate_height(&data)?,
            hair_color: validate_hair_color(&data)?,
            eye_color: validate_eye_color(&data)?,
            passport_id: validate_passport_id(&data)?,
            country_id: find_passport_data(&data, "cid:").ok(),
        });
    }
}

fn part1(input: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Lines = input.lines();

    let mut count = 0;
    let mut curr = String::from("");
    for line in lines {
        if line.is_empty() {
            if Passport::from_str(curr.as_str()).is_ok() {
                count += 1;
            }
            curr.clear();
            continue;
        }
        // make sure there is whitespace separating
        curr.push_str(" ");
        curr.push_str(line);
    }

    if !curr.is_empty() {
        if Passport::from_str(curr.as_str()).is_ok() {
            count += 1;
        }
    }

    println!("part 1 or 2: {}", count);
    Ok(count)
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    unimplemented!()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("day4_1.txt");
    // part 1
    part1(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_sample() {
        let sample = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";
        let answer = part1(sample);
        assert!(answer.is_ok());
        assert_eq!(answer.unwrap(), 2);
    }
}
