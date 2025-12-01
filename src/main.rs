use std::error;
use std::fs;
use std::fmt;
use std::str;
use std::str::FromStr;

fn main() {
    println!("day1: {:?}", day1("./inputs/day1.txt", 50));
}

enum ParseRotationError {
    InvalidLength,
    InvalidInteger,
    InvalidDirection,
}

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rotation::Left(x) => write!(f, "L{}", x),
            Rotation::Right(x) => write!(f, "R{}", x),
        }
    }
}

impl str::FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() < 2 {
            return  Err(ParseRotationError::InvalidLength);
        }
        let first = s.chars().next();
        let rest = s.chars().skip(1);
        let num = rest.collect::<String>().parse::<i32>().map_err(|_| ParseRotationError::InvalidInteger)?;
        match first {
            Some('L') => Ok(Rotation::Left(num)),
            Some('R') => Ok(Rotation::Right(num)),
            _ => Err(ParseRotationError::InvalidDirection),
        }
    }
}

fn day1(filename: &str, start_position: i32) -> Result<(i32, i32), Box<dyn error::Error>>{
    let data = fs::read_to_string(filename)?;
    let result = data.lines()
        .filter(|s| !s.is_empty())
        .map(|s| Rotation::from_str(s))
        .filter_map(Result::ok)
        .scan(
            (start_position, 0),
            |(position, zeroes), rotation| { 
                let mut p = match rotation {
                    Rotation::Left(num) => *position - num,
                    Rotation::Right(num) => *position + num
                };
                if p <= 0 {
                    if *position != 0 {
                        *zeroes += 1;
                    }
                    *zeroes += p / -100;
                    p = 100 + (p % 100);
                }
                else {
                    *zeroes += p / 100;
                }
                *position = p % 100;
                Some((rotation, *position, *zeroes))
            }
        )
        .collect::<Vec<_>>();
    println!("{:?}", result);
    let part1 = result.iter().filter(|(_, position, _)| *position == 0).count() as i32;
    let part2 = result.iter().map(|(_, _, zeroes)| zeroes).last();
    return Ok((part1, *part2.expect("should not be empty")));
}
