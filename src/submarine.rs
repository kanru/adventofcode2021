pub mod bingo;
pub mod diagnostic;

use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

pub struct Submarine {
    pos: Position,
    control: Box<dyn Control>,
}

struct Position {
    x: i32,
    depth: i32,
    aim: i32,
}

trait Control {
    fn forward(&self, prev: &Position, unit: i32) -> Position;
    fn up(&self, prev: &Position, unit: i32) -> Position;
    fn down(&self, prev: &Position, unit: i32) -> Position;
}

struct ControlV1;
struct ControlV2;

impl Control for ControlV1 {
    fn forward(&self, prev: &Position, unit: i32) -> Position {
        Position {
            x: prev.x + unit,
            ..*prev
        }
    }

    fn up(&self, prev: &Position, unit: i32) -> Position {
        Position {
            depth: prev.depth - unit,
            ..*prev
        }
    }

    fn down(&self, prev: &Position, unit: i32) -> Position {
        Position {
            depth: prev.depth + unit,
            ..*prev
        }
    }
}

impl Control for ControlV2 {
    fn forward(&self, prev: &Position, unit: i32) -> Position {
        Position {
            x: prev.x + unit,
            depth: prev.depth + prev.aim * unit,
            ..*prev
        }
    }

    fn up(&self, prev: &Position, unit: i32) -> Position {
        Position {
            aim: prev.aim - unit,
            ..*prev
        }
    }

    fn down(&self, prev: &Position, unit: i32) -> Position {
        Position {
            aim: prev.aim + unit,
            ..*prev
        }
    }
}

impl Submarine {
    pub fn v1() -> Submarine {
        Submarine {
            pos: Position {
                x: 0,
                depth: 0,
                aim: 0,
            },
            control: Box::new(ControlV1),
        }
    }

    pub fn v2() -> Submarine {
        Submarine {
            pos: Position {
                x: 0,
                depth: 0,
                aim: 0,
            },
            control: Box::new(ControlV2),
        }
    }

    pub fn read_instruction(input: &str) -> io::Result<Vec<String>> {
        let file = File::open(input)?;
        let reader = BufReader::new(file);
        reader.lines().collect()
    }

    pub fn run(&mut self, input: &[String]) {
        for inst in input {
            let mut split = inst.split(' ');
            let dir = split.next().unwrap();
            let unit = split.next().unwrap().parse::<i32>().unwrap();
            match dir {
                "up" => self.pos = self.control.up(&self.pos, unit),
                "down" => self.pos = self.control.down(&self.pos, unit),
                "forward" => self.pos = self.control.forward(&self.pos, unit),
                _ => panic!("unknown instruction {}", dir),
            }
        }
    }

    pub fn report(&self) -> i32 {
        self.pos.x * self.pos.depth
    }
}

#[cfg(test)]
mod tests {
    use super::Submarine;

    #[test]
    fn test_day2_part1() {
        let mut submarine = Submarine::v1();
        submarine.run(&["forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string()]);
        assert_eq!(150, submarine.report());
    }

    #[test]
    fn test_day2_part2() {
        let mut submarine = Submarine::v2();
        submarine.run(&["forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string()]);
        assert_eq!(900, submarine.report());
    }
}
