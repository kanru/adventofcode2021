use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

pub struct Sonar;

impl Sonar {
    pub fn sweep(input: &str) -> io::Result<Vec<i32>> {
        let file = File::open(input)?;
        let reader = BufReader::new(file);

        Ok(reader
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect())
    }

    pub fn measure_width(reading: &[i32], window: usize) -> usize {
        reading
            .windows(window)
            .map(|s| s.iter().sum())
            .collect::<Vec<_>>()
            .windows(2)
            .filter(|s: &&[i32]| s[1] > s[0])
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::Sonar;

    #[test]
    fn measure_one() {
        assert_eq!(3, Sonar::measure_width(&[1, 2, 3, 2, 4], 1));
    }

    #[test]
    fn measure_three() {
        assert_eq!(
            5,
            Sonar::measure_width(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263], 3)
        );
    }
}
