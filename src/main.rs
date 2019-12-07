use std::env;
use std::io::{Error, ErrorKind};
use std::vec::{Vec};

use aoc19::riddle::riddlecontainer::{RiddleContainer};

fn main() -> Result<(), std::io::Error> {
    let riddles = RiddleContainer::new();

    let args = {
        let args : Vec<String> = env::args().collect();
        if args.len() <= 1 {
            return Err(Error::new(ErrorKind::Other, "expected at least one argument"))
        }
        args
    };

    let riddle = args[1].to_string();
    let remaining_args = if args.len() == 1 {
        Vec::<String>::new()
    } else {
        args[1..].to_vec()
    };

    println!("Hello, world! {:?}", args);

    let result = riddles.solve_riddle(riddle, &remaining_args)?;

    println!("result: {:?}", result);

    Ok(())
}
