use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    rc::Rc,
};

pub struct OctopusSim {
    flashes: Rc<Cell<usize>>,
    octopus: Vec<Vec<Rc<RefCell<Octopus>>>>,
}

#[derive(Clone)]
struct Octopus {
    energy: u8,
    state: OctopusState,
    neighbors: Vec<Rc<RefCell<Octopus>>>,
    flashes: Rc<Cell<usize>>,
}

#[derive(PartialEq, Clone, Copy)]
enum OctopusState {
    Flashed,
    Normal,
}

impl Octopus {
    fn with_energy(energy: u8, flashes: Rc<Cell<usize>>) -> Octopus {
        Octopus {
            energy,
            flashes,
            state: OctopusState::Normal,
            neighbors: vec![],
        }
    }
    fn step(&mut self) {
        self.state = OctopusState::Normal;
        self.energy += 1;
    }
    fn react(&mut self) -> Vec<Rc<RefCell<Octopus>>> {
        if self.energy > 9 {
            self.flash()
        } else {
            vec![]
        }
    }
    fn flash(&mut self) -> Vec<Rc<RefCell<Octopus>>> {
        self.flashes.set(self.flashes.get() + 1);
        self.energy = 0;
        self.state = OctopusState::Flashed;
        let mut to_charge = vec![];
        for o in &self.neighbors {
            if o.borrow().state != OctopusState::Flashed {
                to_charge.push(Rc::clone(o));
            }
        }
        to_charge
    }
    fn charge(&mut self) -> Vec<Rc<RefCell<Octopus>>> {
        let mut to_charge = vec![];
        if self.state != OctopusState::Flashed {
            self.energy += 1;
            if self.energy > 9 {
                to_charge = self.flash();
            }
        }
        to_charge
    }
}

impl OctopusSim {
    pub fn init_from_file(path: &str) -> io::Result<OctopusSim> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        OctopusSim::init_from_str(&buf)
    }
    fn init_from_str(input: &str) -> io::Result<OctopusSim> {
        let reader = BufReader::new(input.as_bytes());
        let flashes = Rc::new(Cell::new(0));
        let octopus: Vec<Vec<_>> = reader
            .lines()
            .map(|line| line.unwrap())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .map(|e| Rc::new(RefCell::new(Octopus::with_energy(e, Rc::clone(&flashes)))))
                    .collect()
            })
            .collect();
        let height = octopus.len();
        let width = octopus[0].len();
        for i in 0..height {
            for j in 0..width {
                for &u in &[-1isize, 0, 1] {
                    for &v in &[-1isize, 0, 1] {
                        let neighbor_i = i.overflowing_add(u as usize).0;
                        let neighbor_j = j.overflowing_add(v as usize).0;
                        if u == 0 && v == 0 {
                            continue;
                        }
                        if neighbor_i >= height {
                            continue;
                        }
                        if neighbor_j >= width {
                            continue;
                        }
                        RefCell::borrow_mut(&octopus[i][j])
                            .neighbors
                            .push(Rc::clone(&octopus[neighbor_i][neighbor_j]));
                    }
                }
            }
        }
        Ok(OctopusSim { octopus, flashes })
    }
    pub fn run(&self, turns: usize) -> usize {
        let mut octopus = self.octopus.clone();
        self.flashes.set(0);

        for _ in 0..turns {
            for row in &mut octopus {
                for o in row {
                    RefCell::borrow_mut(&o).step();
                }
            }
            let mut to_charge = VecDeque::new();
            for row in &mut octopus {
                for o in row {
                    to_charge.extend(RefCell::borrow_mut(&o).react().into_iter());
                }
            }
            while !to_charge.is_empty() {
                for _ in 0..to_charge.len() {
                    let o = to_charge.pop_front().expect("empty queye");
                    to_charge.extend(RefCell::borrow_mut(&o).charge().into_iter());
                }
            }
        }

        self.flashes.get()
    }

    pub fn run_til_sync(&self) -> usize {
        let mut octopus = self.octopus.clone();
        let mut steps = 0;
        self.flashes.set(0);

        loop {
            steps += 1;
            let prev = self.flashes.get();
            for row in &mut octopus {
                for o in row {
                    RefCell::borrow_mut(&o).step();
                }
            }
            let mut to_charge = VecDeque::new();
            for row in &mut octopus {
                for o in row {
                    to_charge.extend(RefCell::borrow_mut(&o).react().into_iter());
                }
            }
            while !to_charge.is_empty() {
                for _ in 0..to_charge.len() {
                    let o = to_charge.pop_front().expect("empty queye");
                    to_charge.extend(RefCell::borrow_mut(&o).charge().into_iter());
                }
            }
            if self.flashes.get() - prev == self.octopus.len() * self.octopus[0].len() {
                break;
            }
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::OctopusSim;

    #[test]
    fn test_day11() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let sim = OctopusSim::init_from_str(input).expect("parse error");
        sim.run(100);
        assert_eq!(1656, sim.flashes.get());
    }

    #[test]
    fn test_day11_part2() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let sim = OctopusSim::init_from_str(input).expect("parse error");
        assert_eq!(195, sim.run_til_sync());
    }
}
