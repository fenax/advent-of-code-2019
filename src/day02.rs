use std::vec::*;
use advent_of_code_2019::*;

pub struct Day02Input{
    data:Vec<i32>,
}

impl ParsedInput for Day02Input{
    fn parse(input:& String)->Day02Input{
        Day02Input{
            data :
                input.split(',')
                     .map(str::trim)
                     .filter_map(|s|s.parse::<i32>().ok())
                     .collect()
        }
    }
}

impl Day02Input{
    pub fn parametrize(&mut self, noun:i32,verb:i32){
        self.data[1] = noun;
        self.data[2] = verb;
    }
}

impl std::fmt::Display for Day02Input{
    fn fmt(&self,f: &mut std::fmt::Formatter)->std::fmt::Result{
        for v in self.data.iter(){
            write!(f,"{},",v)?;
        }
        Ok(())
    }
}

pub struct Day02Puzzle{
    value:i32,
}

fn run_program(mut ram:Vec<i32>)->i32{
    let mut pc:usize = 0;
    loop{
        let mut arg1 = ram[ram[pc+1] as usize];
        let mut arg2 = ram[ram[pc+2] as usize];
        let mut arg3 = ram[ram[pc+3] as usize];        
        match run_instruction(&ram[pc],&mut arg1,&mut arg2,&mut arg3){
            true => {
                let position = ram[pc+3] as usize;
                ram[position]= arg3;
                pc += 4;
            },
            false => return ram[0],
        }
    }
}

fn run_instruction(op:&i32,arg1:&mut i32,arg2:&mut i32,arg3:&mut i32)->bool{
    match op{
        1 => {
            // ADD
            *arg3 = *arg1 + *arg2;
            true
        },
        2 => {
            // MUL
            *arg3 = *arg1 * *arg2;
            true
        },
        99 =>{
            false
        },
        _ => panic!("unknown opcode"),
    }
}

impl PuzzleSolution<Day02Input> for Day02Puzzle{
    
    fn solve(input:&Day02Input)->Day02Puzzle{
        Day02Puzzle{
            value : run_program(input.data.to_owned())
        }
    }
}
impl std::fmt::Display for Day02Puzzle{
    fn fmt(&self,f: &mut std::fmt::Formatter)->std::fmt::Result{
        write!(f,"Day 02 Part One : {}",self.value)
    }
}



pub struct Day02PuzzleTwo{
    value:Option<i32>,
}
 

impl PuzzleSolution<Day02Input> for Day02PuzzleTwo{
    fn solve(input:&Day02Input)->Day02PuzzleTwo{
        for i in 0..=99{
            for j in 0..=99{
                let mut data = input.data.to_owned();
                data[1] = i;
                data[2] = j;
                let result = run_program(data);
                if result == 19690720{
                    return Day02PuzzleTwo{value:Some(i*100+j)};
                }
            }
        }
        Day02PuzzleTwo{value:None}
    }
}
impl std::fmt::Display for Day02PuzzleTwo{
    fn fmt(&self,f: &mut std::fmt::Formatter)->std::fmt::Result{
        write!(f,"Day 02 Part Two : {}",
        match self.value{
            None => "No result found".to_string(),
            Some(x) => x.to_string()
        })
    }
}


