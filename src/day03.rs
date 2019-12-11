use std::vec::*;
use advent_of_code_2019::*;
 
enum Orientation{
    Vertical,
    Horizontal,
};

struct Segment{
    orientation:Orientation,
    origin_x:i32,
    origin_y:i32,
    lenght:i32,
}
/*
impl Segment{
    pub fn collide(&self,&other:Segment){
        if self.orientation == other.orientation{

            match self.orientation{
                Orientation::Horizontal=>{},
                Orientation::Vertical=>{},
            }
        }
    }
}
*/
pub struct Day03Input{
    wire1:Vec<Segment>,
    wire2:Vec<Segment>
}


fn parse_line(line:&Vec<&str>)->Vec<Segment>{
    let mut node_x = 0;
    let mut node_y = 0;
    line.split(',')
        .map(str::trim)
        .filter_map(|s|(s[0], s[1..].parse::<i32>().ok()))
        .map(|(d,l)|
            let ret = 
                match d{
                    'U' => Segment{    orientation:Orientation::Vertical, 
                                    origin_y:node_y - l,
                                    origin_x:node_x,
                                    length:l},
                    'D' => Segment{    orientation:Orientation::Vertical,
                                    origin_y:node_y,
                                    origin_x:node_x,
                                    length:l},
                    'L' => Segment{    orientation:Orientation::Horizontal,
                                    origin_y:node_y,
                                    origin_x:node_x - l,
                                    lenght : l},
                    'R' => Segment{    orientation:Orientation::Horizontal,
                                    origin_y:node_y,
                                    origin_x:node_x,
                                    length :l},  
                };
            match d{
                'U' => node_y = node_y - l,
                'D' => node_y = node_y + l,
                'L' => node_x = node_x - l,
                'R' => node_x = node_x + l,
            };
            ret)
        .collect();
}

impl ParsedInput for Day03Input{
    fn parse(input:& String)->Day03Input{
        let lines: Vec<&str> = input.split('\n').map(str::trim).collect();

        Day03Input{
            wire1: parse_line(lines[0]),
            wire2: parse_line(lines[1]),
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


