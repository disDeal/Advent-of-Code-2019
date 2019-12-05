use std::collections::HashMap;
use std::convert::TryFrom;
use std::io;

use crate::error::Error;
use crate::{bail, error};

const ORIGIN: Point = Point(0, 0);

type State = HashMap<Point, [bool; 2]>;

pub fn run<R>(input: R) -> Result<(String, String), Error>
where
    R: io::BufRead,
{
    let mut state: State = HashMap::new();

    let mut id = 0;

    for res in input.lines() {
        assert!(id < 2);
        let line = res?;

        let mut point = ORIGIN;

        for s in line.trim().split(',').map(|s| s.trim()) {
            let instruction = s.parse::<Instruction>()?;
            let new_point = process_instruction(id, point, instruction, &mut state);
            point = new_point;
        }

        id += 1;
    }

    let answer =
        state
            .iter()
            .filter(|(_, v)| v[0] && v[1])
            .fold(std::u32::MAX, |mut min, (&point, _)| {
                let dist = manhattan_distance(point, ORIGIN);

                if dist < min {
                    min = dist;
                }
                min
            });

    Ok((format!("{}", answer), "bar".to_string()))
}

fn process_instruction(
    id: usize,
    origin: Point,
    instruction: Instruction,
    state: &mut State,
) -> Point {
    let (i, j) = match instruction.dir {
        Direction::U => (0, 1),
        Direction::D => (0, -1),
        Direction::L => (1, 0),
        Direction::R => (-1, 0),
    };

    let mut destination = origin;
    for num in 1..=instruction.dist {
        let point = Point(origin.0 + num as i32 * i, origin.1 + num as i32 * j);
        let value = state.entry(point).or_insert_with(|| [false, false]);
        value[id] = true;
        destination = point;
    }

    destination
}

fn manhattan_distance(a: Point, b: Point) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

struct Instruction {
    dir: Direction,
    dist: u32,
}

impl std::str::FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let dir = Direction::try_from(bytes[0] as char)?;
        let dist = atoi::atoi::<u32>(&bytes[1..])
            .ok_or_else(|| error!("Unable parse {} into an instruction", s))?;
        Ok(Instruction { dir, dist })
    }
}

enum Direction {
    U,
    D,
    L,
    R,
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let direction = match c {
            'U' => Direction::U,
            'D' => Direction::D,
            'L' => Direction::L,
            'R' => Direction::R,
            _ => bail!("Unable to parse {} unto a direction", c),
        };

        Ok(direction)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point(i32, i32);

#[cfg(test)]
mod tests {
    #[test]
    fn test_03() {}
}
