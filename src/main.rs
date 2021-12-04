mod sonar;
mod submarine;

use sonar::Sonar;
use std::io;
use submarine::{bingo::BingoSystem, diagnostic::DiagnosticModule, Submarine};

fn main() -> io::Result<()> {
    let input = "data/day01.txt";
    let reading = Sonar::sweep(input)?;
    println!("Day 1, part 1 => {}", Sonar::measure_width(&reading, 1));
    println!("Day 1, part 2 => {}", Sonar::measure_width(&reading, 3));

    let input = "data/day02.txt";
    let mut submarine_v1 = Submarine::v1();
    let mut submarine_v2 = Submarine::v2();
    let inst = Submarine::read_instruction(&input)?;
    submarine_v1.run(&inst);
    submarine_v2.run(&inst);
    println!("Day 2, part 1 => {}", submarine_v1.report());
    println!("Day 2, part 2 => {}", submarine_v2.report());

    let input = "data/day03.txt";
    let diagnostic_module = DiagnosticModule::from_file(&input)?;
    println!("Day 3, part 1 => {:?}", diagnostic_module.generate_report());

    let input = "data/day04.txt";
    let bingo_system = BingoSystem::from_file(&input)?;
    println!("Day 4, part 1 => {}", bingo_system.bingo_to_win());
    println!("Day 4, part 2 => {}", bingo_system.bingo_to_lose());

    Ok(())
}
