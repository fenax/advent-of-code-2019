extern crate advent_of_code_2019;
extern crate rayon;

use crate::advent_of_code_2019::*;
use std::fs::read_to_string;
mod day01;
mod day02;

fn main() -> Result<(), std::io::Error> {

{
    let input_day01 = read_to_string("day01.input")?;
    let parsed_input_day01 = day01::Day01Input::parse(&input_day01);
    let solution_day01 = day01::Day01Puzzle::solve(&parsed_input_day01);
    println!("{}",&solution_day01);
    let solution_day01_two = day01::Day01PuzzleTwo::solve(&parsed_input_day01);
    println!("{}",&solution_day01_two);
}
{
    let input_day02 = read_to_string("day02.input")?;
    let mut parsed_input_day02 = day02::Day02Input::parse(&input_day02);
    parsed_input_day02.parametrize(12, 02);
    let solution_day02 = day02::Day02Puzzle::solve(&parsed_input_day02);
    println!("{}",&solution_day02);
        
    let solution_day02_two = day02::Day02PuzzleTwo::solve(&parsed_input_day02);
    println!("{}",&solution_day02_two);
}
    Ok(())
}
