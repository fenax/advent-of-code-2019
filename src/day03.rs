use std::vec::*;
use advent_of_code_2019::*;
use std::cmp::{max,min};
 
#[derive(std::cmp::PartialEq)]
enum Orientation{
    Vertical,
    Horizontal,
}

struct Segment{
    orientation:Orientation,
    origin_x:i32,
    origin_y:i32,
    length:i32,
}

impl Segment{
    pub fn collide(&self,other:&Segment)->Option<(i32,i32)>{
        if self.orientation == other.orientation{
            if match self.orientation{
                Orientation::Horizontal=>self.origin_y == other.origin_y,
                Orientation::Vertical=>self.origin_x == other.origin_x
            }{
                
            }
        }
        None
    }

    pub fn get_topleft(&self)->(i32,i32){
        (self.origin_x,self.origin_y)
    }
    pub fn get_bottomright(&self)->(i32,i32){
        match self.orientation{
            Orientation::Horizontal=>(self.origin_x + self.length,self.origin_y),
            Orientation::Vertical=>(self.origin_x,self.origin_y+self.length),
        }
    }
}


fn get_maxima(path : &Vec<Segment>)->((i32,i32),(i32,i32)){
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;

    for s in path{
        let (x,y) = s.get_topleft();
        x_min = min(x_min,x);
        y_min = min(y_min,y);
        let (x,y) = s.get_bottomright();
        x_max = max(x_max,x);
        y_max = max(y_max,y);
    }

    ((x_min,x_max),(y_min,y_max))
}
pub struct Day03Input{
    wire1:Vec<Segment>,
    wire2:Vec<Segment>
}


fn parse_line(line:&str)->Vec<Segment>{
    let mut node_x = 0;
    let mut node_y = 0;
    line.split(',')
        .map(str::trim)
        .map(|s|(s.chars().next().unwrap() , s[1..].parse::<i32>().expect("this is wrong")))
        .map(|(d,l)|{
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
                                    length : l},
                    'R' => Segment{    orientation:Orientation::Horizontal,
                                    origin_y:node_y,
                                    origin_x:node_x,
                                    length :l},
                    _ => panic!("nonsense"),
                };
            match d{
                'U' => node_y = node_y - l,
                'D' => node_y = node_y + l,
                'L' => node_x = node_x - l,
                'R' => node_x = node_x + l,
                _ => panic!("should already have panic ed"),
            };
            ret}
            )
        .collect()
}




impl ParsedInput for Day03Input{
    fn parse(input:& String)->Day03Input{
        let lines: Vec<&str> = input.split('\n').map(str::trim).collect();
        let wire1 = parse_line(lines[0]);
        let wire2 = parse_line(lines[1]);
        

        Day03Input{
            wire1,
            wire2,
        }
    }
}
impl std::fmt::Display for Day03Input{
    fn fmt(&self,f: &mut std::fmt::Formatter)->std::fmt::Result{
        let ((x_min,x_max),(y_min,y_max)) = get_maxima(&self.wire1);
        write!(f,"{} items between X {},{} Y {},{}\n",self.wire1.len(),x_min,x_max,y_min,y_max)?;
        let ((x_min,x_max),(y_min,y_max)) = get_maxima(&self.wire2);
        write!(f,"{} items between X {},{} Y {},{}\n",self.wire2.len(),x_min,x_max,y_min,y_max)?;
        Ok(())
    }
}
/*
pub struct Day03Puzzle{
    value:i32,
}

impl PuzzleSolution<Day03Input> for Day03Puzzle{
    
    fn solve(input:&Day03Input)->Day03Puzzle{
        Day03Puzzle{
            value : input.data.iter()
                              .map(secret_calculation)
                              .sum()
        }
    }
}

impl std::fmt::Display for Day03Puzzle{
    fn fmt(&self,f: &mut std::fmt::Formatter)->std::fmt::Result{
        write!(f,"Day 01 Part One : {}",self.value)
    }
}


pub struct Day03PuzzleTwo{
    value:i32,
}
 

impl PuzzleSolution<Day03Input> for Day03PuzzleTwo{
    
    fn solve(input:&Day03Input)->Day03PuzzleTwo{
        Day03PuzzleTwo{
            value : input.data.iter()
                              .map(secret_calculation_plus_fuel)
                              .sum()
        }
    }
}
impl std::fmt::Display for Day03PuzzleTwo{
    fn fmt(&self,f: &mut std::fmt::Formatter)->std::fmt::Result{
        write!(f,"Day 01 Part Two : {}",self.value)
    }
}
*/

