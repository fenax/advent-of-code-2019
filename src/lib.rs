
pub trait ParsedInput{
    fn parse(input:&String)->Self;
}

pub trait PuzzleSolution<I:ParsedInput>{
    fn solve(input:&I)->Self;
}

