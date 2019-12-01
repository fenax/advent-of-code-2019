use std::vec::*;
use advent_of_code_2019::*;

pub struct Day01Input{
    data:Vec<i32>,
}

impl ParsedInput for Day01Input{
    fn parse(input:& String)->Day01Input{
        Day01Input{
            data :
                input.split('\n')
                     .map(str::trim)
                     .filter_map(|s|s.parse::<i32>().ok())
                     .collect()
        }
    }
}
impl std::fmt::Display for Day01Input{
    fn fmt(&self,f: &mut std::fmt::Formatter)->std::fmt::Result{
        for v in self.data.iter(){
            write!(f,"{},",v)?;
        }
        Ok(())
    }
}

pub struct Day01Puzzle{
    value:i32,
}

fn secret_calculation(v:&i32)->i32{
    (v/3) - 2
}


impl PuzzleSolution<Day01Input> for Day01Puzzle{
    
    fn solve(input:&Day01Input)->Day01Puzzle{
        Day01Puzzle{
            value : input.data.iter()
                              .map(secret_calculation)
                              .sum()
        }
    }
}
impl std::fmt::Display for Day01Puzzle{
    fn fmt(&self,f: &mut std::fmt::Formatter)->std::fmt::Result{
        write!(f,"Day 01 Part One : {}",self.value)
    }
}

fn secret_calculation_plus_fuel(v:&i32)->i32{
    let mut sum:i32 = 0;
    let mut current = *v;
    loop{
        let new = secret_calculation(&current);
        if new <= 0 {
            return sum
        }else{
            sum += new;
            current = new;
        }
    }
}

pub struct Day01PuzzleTwo{
    value:i32,
}
 

impl PuzzleSolution<Day01Input> for Day01PuzzleTwo{
    
    fn solve(input:&Day01Input)->Day01PuzzleTwo{
        Day01PuzzleTwo{
            value : input.data.iter()
                              .map(secret_calculation_plus_fuel)
                              .sum()
        }
    }
}
impl std::fmt::Display for Day01PuzzleTwo{
    fn fmt(&self,f: &mut std::fmt::Formatter)->std::fmt::Result{
        write!(f,"Day 01 Part Two : {}",self.value)
    }
}


