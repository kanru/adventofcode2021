use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

pub struct BingoSystem {
    seq: Vec<i32>,
    boards: Vec<FiveByFiveBoard>,
}

impl BingoSystem {
    pub fn from_file(input: &str) -> io::Result<BingoSystem> {
        let mut file = File::open(input)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        BingoSystem::from_str(&buf)
    }

    pub fn from_str(input: &str) -> io::Result<BingoSystem> {
        let mut reader = BufReader::new(input.as_bytes());
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        let seq = buf
            .trim_end()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();

        // Read boards
        let mut boards = vec![];
        let mut numbers = vec![];
        loop {
            numbers.clear();
            buf.clear();
            if reader.read_line(&mut buf)? == 0 {
                break;
            }

            for _ in 0..5 {
                buf.clear();
                reader.read_line(&mut buf)?;
                numbers.extend(
                    buf.trim()
                        .split(" ")
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse::<i32>().unwrap()),
                )
            }
            boards.push(FiveByFiveBoard::with_numbers(&numbers));
        }

        Ok(BingoSystem { seq, boards })
    }

    pub fn bingo_to_win(&self) -> i32 {
        let mut boards = self.boards.clone();
        for &number in &self.seq {
            for board in &mut boards {
                board.check_and_mark(number);
                if board.is_win() {
                    return board.score(number);
                }
            }
        }
        unreachable!("must have a winner")
    }

    pub fn bingo_to_lose(&self) -> i32 {
        let mut boards = self.boards.clone();
        let mut last_win_board = FiveByFiveBoard::new();
        let mut last_win_num = 0;
        for &number in &self.seq {
            boards = boards
                .iter_mut()
                .map(|board| {
                    board.check_and_mark(number);
                    board
                })
                .filter(|&&mut board| {
                    if board.is_win() {
                        last_win_board = board;
                        last_win_num = number;
                        false
                    } else {
                        true
                    }
                })
                .map(|b| *b)
                .collect();
        }
        last_win_board.score(last_win_num)
    }
}

#[derive(Clone, Copy)]
struct FiveByFiveBoard {
    numbers: [i32; 25],
    markers: [bool; 25],
}

impl FiveByFiveBoard {
    pub fn new() -> FiveByFiveBoard {
        FiveByFiveBoard {
            numbers: [0; 25],
            markers: [false; 25],
        }
    }

    pub fn with_numbers(numbers: &[i32]) -> FiveByFiveBoard {
        FiveByFiveBoard {
            numbers: numbers.try_into().expect("wrong numbers length"),
            markers: [false; 25],
        }
    }

    fn is_win(&self) -> bool {
        // Check rows first
        for r in 0..5 {
            let mut win = true;
            for col in 0..5 {
                if self.markers[r * 5 + col] == false {
                    win = false;
                    break;
                }
            }
            if win {
                return true;
            }
        }
        // Now check columns
        for col in 0..5 {
            let mut win = true;
            for r in 0..5 {
                if self.markers[r * 5 + col] == false {
                    win = false;
                    break;
                }
            }
            if win {
                return true;
            }
        }
        false
    }

    fn check_and_mark(&mut self, number: i32) {
        if let Some(index) =
            self.numbers
                .iter()
                .enumerate()
                .find_map(|(i, &val)| if val == number { Some(i) } else { None })
        {
            self.markers[index] = true;
        }
    }

    fn score(&self, called: i32) -> i32 {
        let mut sum = 0;
        for r in 0..5 {
            for col in 0..5 {
                if !self.markers[r * 5 + col] {
                    sum += self.numbers[r * 5 + col];
                }
            }
        }
        sum * called
    }
}

#[cfg(test)]
mod tests {
    use super::{BingoSystem, FiveByFiveBoard};

    #[test]
    fn test_winning() {
        let board = FiveByFiveBoard {
            numbers: [0; 25],
            #[rustfmt::skip]
            markers: [
                false, false, false, false, false,
                 true,  true,  true,  true,  true,
                false, false, false, false, false,
                false, false, false, false, false,
                false, false, false, false, false,
            ],
        };
        assert!(board.is_win());

        let board = FiveByFiveBoard {
            numbers: [0; 25],
            #[rustfmt::skip]
            markers: [
                false, true, false, false, false,
                false, true, false, false, false,
                false, true, false, false, false,
                false, true, false, false, false,
                false, true, false, false, false,
            ],
        };
        assert!(board.is_win());
    }

    #[test]
    fn test_no_winning() {
        let board = FiveByFiveBoard {
            numbers: [0; 25],
            #[rustfmt::skip]
            markers: [
                false, false, false, false, false,
                true, false, true, true, true,
                false, false, false, false, false,
                false, false, false, false, false,
                false, false, false, false, false,
            ],
        };
        assert!(!board.is_win());

        let board = FiveByFiveBoard {
            numbers: [0; 25],
            #[rustfmt::skip]
            markers: [
                false, true, false, false, false,
                false, true, false, false, false,
                false, false, false, false, false,
                false, true, false, false, false,
                false, true, false, false, false,
            ],
        };
        assert!(!board.is_win());
    }

    #[test]
    fn test_mark_some_numbers() {
        let mut board = FiveByFiveBoard {
            #[rustfmt::skip]
            numbers: [
                22, 13, 17, 11,  0,
                 8,  2, 23,  4, 24,
                21,  9, 14, 16,  7,
                 6, 10,  3, 18,  5,
                 1, 12, 20, 15, 19,
            ],
            markers: [false; 25],
        };
        board.check_and_mark(14);
        board.check_and_mark(19);
        assert_eq!(true, board.markers[12]);
        assert_eq!(true, board.markers[24]);
    }

    #[test]
    fn test_day4() {
        let bingo_system = BingoSystem::from_str(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19
            
             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6
            
            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7",
        )
        .expect("fail to create a bingo system");
        assert_eq!(4512, bingo_system.bingo_to_win());
        assert_eq!(1924, bingo_system.bingo_to_lose());
    }
}
