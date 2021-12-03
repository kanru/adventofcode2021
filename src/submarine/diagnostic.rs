use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub struct DiagnosticModule {
    reading: Vec<Vec<u8>>,
}

#[derive(PartialEq, Debug)]
pub struct DiagnosticReport {
    pub power_consumption: i32,
    pub life_support_rating: i32,
}

impl DiagnosticModule {
    pub fn new() -> DiagnosticModule {
        DiagnosticModule { reading: vec![] }
    }

    pub fn from_file(input: &str) -> io::Result<DiagnosticModule> {
        let file = File::open(input)?;
        let reader = BufReader::new(file);

        Ok(DiagnosticModule {
            reading: reader
                .lines()
                .map(|line| line.unwrap().into_bytes())
                .collect(),
        })
    }

    pub fn take_reading(&mut self, reading: Vec<Vec<u8>>) {
        self.reading = reading;
    }

    pub fn generate_report(&self) -> DiagnosticReport {
        let (gamma, epsilon) = self.power_consumption_report();
        let oxygen = self.oxygen_report();
        let co2 = self.co2_report();

        DiagnosticReport {
            power_consumption: gamma * epsilon,
            life_support_rating: oxygen * co2,
        }
    }

    fn power_consumption_report(&self) -> (i32, i32) {
        let mut reading = vec![];
        for line in &self.reading {
            reading.push(line);
        }
        let mut gamma = 0;
        let mut epsilon = 0;
        for i in 0..self.reading[0].len() {
            let most_common = self.most_common(&reading, i).unwrap();
            if most_common == b'1' {
                gamma |= 1;
            } else {
                epsilon |= 1;
            }
            gamma <<= 1;
            epsilon <<= 1;
        }
        gamma >>= 1;
        epsilon >>= 1;
        (gamma, epsilon)
    }

    fn most_common(&self, reading: &Vec<&Vec<u8>>, i: usize) -> Option<u8> {
        let mut ones = 0;
        let mut zeros = 0;
        for line in reading {
            if line[i] == b'1' {
                ones += 1;
            } else {
                zeros += 1;
            }
        }
        if ones > zeros {
            Some(b'1')
        } else if ones < zeros {
            Some(b'0')
        } else {
            None
        }
    }

    fn oxygen_report(&self) -> i32 {
        let mut reading = vec![];
        for line in &self.reading {
            reading.push(line);
        }
        for i in 0..self.reading[0].len() {
            let most_common = self.most_common(&reading, i).unwrap_or(b'1');
            let mut collector = vec![];
            for line in &reading {
                if line[i] == most_common {
                    collector.push(*line);
                }
            }
            reading = collector;
            if reading.len() == 1 {
                break;
            }
        }
        let mut oxygen = 0;
        for &b in reading[0] {
            oxygen |= if b == b'1' { 1 } else { 0 };
            oxygen <<= 1;
        }
        oxygen >> 1
    }

    fn co2_report(&self) -> i32 {
        let mut reading = vec![];
        for line in &self.reading {
            reading.push(line);
        }
        for i in 0..self.reading[0].len() {
            let most_common = self.most_common(&reading, i).unwrap_or(b'1');
            let least_common = if most_common == b'1' { b'0' } else { b'1' };
            let mut collector = vec![];
            for line in &reading {
                if line[i] == least_common {
                    collector.push(*line);
                }
            }
            reading = collector;
            if reading.len() == 1 {
                break;
            }
        }
        let mut co2 = 0;
        for &b in reading[0] {
            co2 |= if b == b'1' { 1 } else { 0 };
            co2 <<= 1;
        }
        co2 >> 1
    }
}

#[cfg(test)]
mod tests {
    use super::{DiagnosticModule, DiagnosticReport};

    #[test]
    fn test_day3() {
        let mut diagnostic = DiagnosticModule::new();
        diagnostic.take_reading(vec![
            b"00100".to_vec(),
            b"11110".to_vec(),
            b"10110".to_vec(),
            b"10111".to_vec(),
            b"10101".to_vec(),
            b"01111".to_vec(),
            b"00111".to_vec(),
            b"11100".to_vec(),
            b"10000".to_vec(),
            b"11001".to_vec(),
            b"00010".to_vec(),
            b"01010".to_vec(),
        ]);
        assert_eq!(
            DiagnosticReport {
                power_consumption: 198,
                life_support_rating: 230,
            },
            diagnostic.generate_report()
        );
    }
}
