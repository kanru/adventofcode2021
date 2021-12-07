use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

pub struct CrabSwarm {
    positions: HashMap<usize, usize>,
}

impl CrabSwarm {
    pub fn init_from_file(path: &str) -> io::Result<CrabSwarm> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        CrabSwarm::new(&buf)
    }

    pub fn new(input: &str) -> io::Result<CrabSwarm> {
        let mut positions = HashMap::new();
        for pos in input.trim().split(',').map(|s| s.parse::<usize>().unwrap()) {
            *positions.entry(pos).or_default() += 1;
        }
        Ok(CrabSwarm { positions })
    }

    pub fn best_alignment(&self) -> usize {
        let mut min = usize::MAX;
        for &pos in self.positions.keys() {
            let mut cand = 0;
            for (p, v) in self.positions.iter() {
                match pos.cmp(p) {
                    Ordering::Greater => cand += v * (pos - p),
                    Ordering::Less => cand += v * (p - pos),
                    Ordering::Equal => (),
                }
            }
            if cand < min {
                min = cand;
            }
        }
        min
    }

    pub fn best_alignment_for_crab_engine(&self) -> usize {
        let &left = self.positions.keys().min().unwrap();
        let &right = self.positions.keys().max().unwrap();
        let mut min = usize::MAX;
        for pos in left..=right {
            let mut cand = 0;
            for (p, v) in self.positions.iter() {
                match pos.cmp(p) {
                    Ordering::Greater => cand += v * crab_engine_fuel_cost(pos - p),
                    Ordering::Less => cand += v * crab_engine_fuel_cost(p - pos),
                    Ordering::Equal => (),
                }
            }
            if cand < min {
                min = cand;
            }
        }
        min
    }
}

fn crab_engine_fuel_cost(changes: usize) -> usize {
    ((1 + changes) * changes) / 2
}

#[cfg(test)]
mod tests {
    use super::{crab_engine_fuel_cost, CrabSwarm};

    #[test]
    fn test_day7() {
        let crab_swarm = CrabSwarm::new("16,1,2,0,4,2,7,1,2,14").expect("parse error");
        assert_eq!(37, crab_swarm.best_alignment());
        assert_eq!(168, crab_swarm.best_alignment_for_crab_engine());
    }

    #[test]
    fn test_crab_engine_fuel_cost() {
        assert_eq!(66, crab_engine_fuel_cost(16 - 5));
        assert_eq!(10, crab_engine_fuel_cost(5 - 1));
        assert_eq!(6, crab_engine_fuel_cost(5 - 2));
        assert_eq!(15, crab_engine_fuel_cost(5 - 0));
        assert_eq!(45, crab_engine_fuel_cost(14 - 5));
    }
}
