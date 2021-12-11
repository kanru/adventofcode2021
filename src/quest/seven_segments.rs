use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

pub struct SevenSegments;

impl SevenSegments {
    pub fn input_from_file(path: &str) -> io::Result<Vec<String>> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        SevenSegments::input_from_str(&buf)
    }

    pub fn input_from_str(input: &str) -> io::Result<Vec<String>> {
        let reader = BufReader::new(input.as_bytes());
        Ok(reader.lines().map(|l| l.unwrap()).collect())
    }

    pub fn count_1478(input: &[String]) -> usize {
        let mut counter = 0;
        for line in input {
            let right = line.split('|').nth(1).unwrap().trim();
            for digit in right.split(' ') {
                match digit.len() {
                    2 | 4 | 3 | 7 => counter += 1,
                    _ => (),
                }
            }
        }
        counter
    }

    pub fn decode_display(input: &[String]) -> usize {
        let mut result = 0;
        for line in input {
            let (left, right) = line.split_once('|').unwrap();
            let mapping = find_mapping(left.trim());
            let display = to_4digit_number(&mapping, &mut right.trim().split(' '));
            result += display;
        }
        result
    }
}

fn find_mapping(input: &str) -> BTreeMap<BTreeSet<char>, u8> {
    let eight = BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    let mut mapping = BTreeMap::new();
    let mut segments = BTreeMap::new();
    for seg in ['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
        segments.insert(seg, eight.clone());
    }
    let observations: Vec<_> = input
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<BTreeSet<char>>())
        .collect();

    for digit in &observations {
        match digit.len() {
            2 => {
                mapping.insert(1, digit.clone());
            }
            4 => {
                mapping.insert(4, digit.clone());
            }
            3 => {
                mapping.insert(7, digit.clone());
            }
            7 => {
                mapping.insert(8, digit.clone());
            }
            _ => (),
        }
    }

    let one = mapping[&1].clone();
    let four = mapping[&4].clone();
    for digit in &observations {
        match (
            digit.len(),
            digit.intersection(&one).count(),
            digit.intersection(&four).count(),
        ) {
            (5, 1, 2) => {
                mapping.insert(2, digit.clone());
            }
            (5, 2, 3) => {
                mapping.insert(3, digit.clone());
            }
            (5, 1, 3) => {
                mapping.insert(5, digit.clone());
            }
            (6, 2, 3) => {
                mapping.insert(0, digit.clone());
            }
            (6, 1, 3) => {
                mapping.insert(6, digit.clone());
            }
            (6, 2, 4) => {
                mapping.insert(9, digit.clone());
            }
            _ => (),
        }
    }

    mapping.into_iter().map(|(k, v)| (v, k)).collect()
}

fn lookup(mapping: &BTreeMap<BTreeSet<char>, u8>, input: &str) -> u8 {
    let input = input.chars().collect::<BTreeSet<_>>();
    mapping[&input]
}

fn to_4digit_number(
    mapping: &BTreeMap<BTreeSet<char>, u8>,
    input: &mut dyn Iterator<Item = &str>,
) -> usize {
    let mut result = 0;
    for digit in input {
        result *= 10;
        result += lookup(mapping, digit) as usize;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{find_mapping, lookup, to_4digit_number, SevenSegments};

    #[test]
    fn test_count_1478() {
        let input = SevenSegments::input_from_str(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce",
        )
        .expect("parse error");
        assert_eq!(26, SevenSegments::count_1478(&input));
    }

    #[test]
    fn test_mapping() {
        let mapping = find_mapping("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab");
        assert_eq!(8, lookup(&mapping, "acedgfb"));
        assert_eq!(5, lookup(&mapping, "cdfbe"));
        assert_eq!(2, lookup(&mapping, "gcdfa"));
        assert_eq!(3, lookup(&mapping, "fbcad"));
        assert_eq!(7, lookup(&mapping, "dab"));
        assert_eq!(9, lookup(&mapping, "cefabd"));
        assert_eq!(6, lookup(&mapping, "cdfgeb"));
        assert_eq!(4, lookup(&mapping, "eafb"));
        assert_eq!(0, lookup(&mapping, "cagedb"));
        assert_eq!(1, lookup(&mapping, "ab"));
        assert_eq!(
            5353,
            to_4digit_number(&mapping, &mut "cdfeb fcadb cdfeb cdbaf".split(' '))
        );
    }

    #[test]
    fn test_decode_display() {
        let input = SevenSegments::input_from_str(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce",
        )
        .expect("parse error");
        assert_eq!(61229, SevenSegments::decode_display(&input));
    }
}
