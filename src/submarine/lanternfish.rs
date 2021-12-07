use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Read},
};

pub struct LanternFishSim {
    pool: VecDeque<usize>,
}

impl LanternFishSim {
    pub fn init_pool_from_file(path: &str) -> io::Result<LanternFishSim> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        LanternFishSim::init_pool(&buf)
    }

    fn init_pool(input: &str) -> io::Result<LanternFishSim> {
        let mut pool: VecDeque<_> = vec![0; 9].into();
        input
            .trim()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .for_each(|num| pool[num] += 1);
        Ok(LanternFishSim { pool })
    }

    pub fn run(&self, days: usize) -> usize {
        let mut pool = self.pool.clone();
        for _ in 0..days {
            pool.rotate_left(1);
            pool[6] += pool[8];
        }
        pool.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::LanternFishSim;

    #[test]
    fn test_day6() {
        let sim = LanternFishSim::init_pool("3,4,3,1,2").expect("init error");
        assert_eq!(5934, sim.run(80));
        assert_eq!(26_984_457_539, sim.run(256));
    }
}
