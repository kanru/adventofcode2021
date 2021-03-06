use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{self, prelude::*, BufReader},
    vec,
};

use regex::Regex;

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

    pub fn heightmap_from_file(path: &str) -> io::Result<HeightMap> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Sonar::heightmap_from_str(&buf)
    }

    fn heightmap_from_str(input: &str) -> io::Result<HeightMap> {
        let stride = input.find('\n').unwrap();
        let buf: Vec<_> = input
            .chars()
            .filter(|&c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let rows = buf.len() / stride;
        Ok(HeightMap { stride, buf, rows })
    }

    pub fn measure_risk_level(height_map: &HeightMap) -> usize {
        let mut risk_level = 0;
        for x in 0..height_map.stride {
            for y in 0..height_map.rows {
                let (center, neighbors) = height_map.probe(x, y);
                if neighbors.iter().all(|&h| h.value > center) {
                    risk_level += center as usize + 1;
                }
            }
        }
        risk_level
    }

    pub fn measure_largest_basin(height_map: &HeightMap) -> usize {
        let mut seen = HashSet::new();
        let mut basin_sizes = Vec::new();
        for x in 0..height_map.stride {
            for y in 0..height_map.rows {
                let (center, neighbors) = height_map.probe(x, y);
                if neighbors.iter().all(|&h| h.value > center) {
                    let mut basin_size = 0;
                    let mut queue = VecDeque::new();
                    queue.push_back((x, y));
                    seen.insert((x, y));
                    while !queue.is_empty() {
                        let (x, y) = queue.pop_front().unwrap();
                        basin_size += 1;
                        let (_, neighbors) = height_map.probe(x, y);
                        for point in &neighbors {
                            if !seen.contains(&point.coord) && point.value < 9 {
                                seen.insert(point.coord);
                                queue.push_back(point.coord);
                            }
                        }
                    }
                    basin_sizes.push(basin_size);
                }
            }
        }
        basin_sizes.sort_unstable();
        basin_sizes.iter().rev().take(3).product()
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

    pub fn scan_hydrothermal_vents_file(path: &str) -> io::Result<Vec<VentLine>> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Sonar::scan_hydrothermal_vents(&buf)
    }

    pub fn scan_hydrothermal_vents(input: &str) -> io::Result<Vec<VentLine>> {
        let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
        let reader = BufReader::new(input.as_bytes());
        let mut vents = vec![];
        for line in reader.lines() {
            let line = line?;
            match re.captures(&line) {
                Some(caps) => vents.push(VentLine {
                    x1: caps.name("x1").unwrap().as_str().parse().unwrap(),
                    x2: caps.name("x2").unwrap().as_str().parse().unwrap(),
                    y1: caps.name("y1").unwrap().as_str().parse().unwrap(),
                    y2: caps.name("y2").unwrap().as_str().parse().unwrap(),
                }),
                None => panic!("Cannot parse line {}", line),
            }
        }
        Ok(vents)
    }

    pub fn simple_count_hydrothermal_active_vents(vent_lines: &[VentLine]) -> usize {
        let mut map: HashMap<(i32, i32), i32> = HashMap::new();
        for line in vent_lines.iter() {
            if line.x1 == line.x2 {
                for y in line.y1.min(line.y2)..=line.y1.max(line.y2) {
                    *map.entry((line.x1, y)).or_default() += 1;
                }
            } else if line.y1 == line.y2 {
                for x in line.x1.min(line.x2)..=line.x1.max(line.x2) {
                    *map.entry((x, line.y1)).or_default() += 1;
                }
            }
        }
        map.values().filter(|&&v| v > 1).count()
    }

    pub fn full_count_hydrothermal_active_vents(vent_lines: &[VentLine]) -> usize {
        let mut map: HashMap<(i32, i32), i32> = HashMap::new();
        for line in vent_lines.iter() {
            let x_off = match line.x1.cmp(&line.x2) {
                Ordering::Greater => -1,
                Ordering::Less => 1,
                Ordering::Equal => 0,
            };
            let y_off = match line.y1.cmp(&line.y2) {
                Ordering::Greater => -1,
                Ordering::Less => 1,
                Ordering::Equal => 0,
            };
            let mut x = line.x1;
            let mut y = line.y1;
            loop {
                *map.entry((x, y)).or_default() += 1;
                if (x, y) == (line.x2, line.y2) {
                    break;
                }
                x += x_off;
                y += y_off;
            }
        }
        map.values().filter(|&&v| v > 1).count()
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct VentLine {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

pub struct HeightMap {
    buf: Vec<u8>,
    stride: usize,
    rows: usize,
}

#[derive(Clone, Copy)]
struct Point {
    value: u8,
    coord: (usize, usize),
}

impl HeightMap {
    fn probe(&self, x: usize, y: usize) -> (u8, Vec<Point>) {
        let mut neighbors = Vec::new();
        let point = x + y * self.stride;
        let center = self.buf[point];
        if x > 0 {
            neighbors.push(Point {
                value: self.buf[point - 1],
                coord: (x - 1, y),
            });
        }
        if x < self.stride - 1 {
            neighbors.push(Point {
                value: self.buf[point + 1],
                coord: (x + 1, y),
            });
        }
        if y > 0 {
            neighbors.push(Point {
                value: self.buf[point - self.stride],
                coord: (x, y - 1),
            })
        }
        if y < self.rows - 1 {
            neighbors.push(Point {
                value: self.buf[point + self.stride],
                coord: (x, y + 1),
            });
        }
        (center, neighbors)
    }
}

#[cfg(test)]
mod tests {
    use super::{Sonar, VentLine};

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

    #[test]
    fn scan_vent_lines() {
        assert_eq!(
            vec![VentLine::default(), VentLine::default()],
            Sonar::scan_hydrothermal_vents("0,0 -> 0,0\n0,0 -> 0,0\n").unwrap()
        );
    }

    #[test]
    fn test_day5_part1() {
        let vent_lines = Sonar::scan_hydrothermal_vents(
            "0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2",
        )
        .unwrap();
        assert_eq!(
            5,
            Sonar::simple_count_hydrothermal_active_vents(&vent_lines)
        );
    }

    #[test]
    fn test_day5_part2() {
        let vent_lines = Sonar::scan_hydrothermal_vents(
            "0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2",
        )
        .unwrap();
        assert_eq!(12, Sonar::full_count_hydrothermal_active_vents(&vent_lines));
    }

    #[test]
    fn test_day9_part1() {
        let height_map = Sonar::heightmap_from_str(
            "2199943210
            3987894921
            9856789892
            8767896789
            9899965678",
        )
        .expect("parse error");
        assert_eq!(15, Sonar::measure_risk_level(&height_map));
    }

    #[test]
    fn test_day9_part2() {
        let height_map = Sonar::heightmap_from_str(
            "2199943210
            3987894921
            9856789892
            8767896789
            9899965678",
        )
        .expect("parse error");
        assert_eq!(1134, Sonar::measure_largest_basin(&height_map));
    }
}
