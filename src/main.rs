mod quest;
mod sonar;
mod submarine;

use sonar::Sonar;
use std::io;
use submarine::{bingo::BingoSystem, diagnostic::DiagnosticModule, Submarine};

use crate::{
    quest::{align_crabs::CrabSwarm, seven_segments::SevenSegments},
    submarine::{
        computer::packet::Packet, lanternfish::LanternFishSim, navigation::NavigationSystem,
        octopus::OctopusSim,
    },
};

fn main() -> io::Result<()> {
    let input = "data/day01.txt";
    let reading = Sonar::sweep(input)?;
    println!("Day 1, part 1 => {}", Sonar::measure_width(&reading, 1));
    println!("Day 1, part 2 => {}", Sonar::measure_width(&reading, 3));

    let input = "data/day02.txt";
    let mut submarine_v1 = Submarine::v1();
    let mut submarine_v2 = Submarine::v2();
    let inst = Submarine::read_instruction(input)?;
    submarine_v1.run(&inst);
    submarine_v2.run(&inst);
    println!("Day 2, part 1 => {}", submarine_v1.report());
    println!("Day 2, part 2 => {}", submarine_v2.report());

    let input = "data/day03.txt";
    let diagnostic_module = DiagnosticModule::from_file(input)?;
    println!("Day 3, part 1 => {:?}", diagnostic_module.generate_report());

    let input = "data/day04.txt";
    let bingo_system = BingoSystem::from_file(input)?;
    println!("Day 4, part 1 => {}", bingo_system.bingo_to_win());
    println!("Day 4, part 2 => {}", bingo_system.bingo_to_lose());

    let input = "data/day05.txt";
    let vent_lines = Sonar::scan_hydrothermal_vents_file(input)?;
    println!(
        "Day 5, part 1 => {}",
        Sonar::simple_count_hydrothermal_active_vents(&vent_lines)
    );
    println!(
        "Day 5, part 2 => {}",
        Sonar::full_count_hydrothermal_active_vents(&vent_lines)
    );

    let input = "data/day06.txt";
    let sim = LanternFishSim::init_pool_from_file(input)?;
    println!("Day 6, part 1 => {}", sim.run(80));
    println!("Day 6, part 1 => {}", sim.run(256));

    let input = "data/day07.txt";
    let crab_swarm = CrabSwarm::init_from_file(input)?;
    println!("Day 7, part 1 => {}", crab_swarm.best_alignment());
    println!(
        "Day 7, part 2 => {}",
        crab_swarm.best_alignment_for_crab_engine()
    );

    let input = "data/day08.txt";
    let input = SevenSegments::input_from_file(input)?;
    println!("Day 8, part 1 => {}", SevenSegments::count_1478(&input));
    println!("Day 8, part 2 => {}", SevenSegments::decode_display(&input));

    let input = "data/day09.txt";
    let height_map = Sonar::heightmap_from_file(input)?;
    println!(
        "Day 9, part 1 => {}",
        Sonar::measure_risk_level(&height_map)
    );
    println!(
        "Day 9, part 2 => {}",
        Sonar::measure_largest_basin(&height_map)
    );

    let input = "data/day10.txt";
    let nav_system = NavigationSystem::boot_from_file(input)?;
    println!(
        "Day 10, part 1 => {}",
        nav_system.calculate_syntax_error_score()
    );
    println!(
        "Day 10, part 2 => {}",
        nav_system.calculate_autocomplete_score()
    );

    let input = "data/day11.txt";
    let sim = OctopusSim::init_from_file(input)?;
    println!("Day 11, part 1 => {}", sim.run(100));
    let sim = OctopusSim::init_from_file(input)?;
    println!("Day 11, part 2 => {}", sim.run_til_sync());

    let input = include_str!("../data/day16.txt");
    let packet = Packet::from_hex(input);
    // packet.dot();
    println!("Day 16, part 1 => {}", packet.version_sum());
    println!("Day 16, part 2 => {}", packet.eval());

    Ok(())
}
