use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

fn error_me(throw: bool) -> Result<()> {
    if throw {
        return Err(anyhow!("this should never be true"));
    }
    std::fs::read(PathBuf::from("/foo")).context("context");

    return Ok(());
}

fn getInput() -> &'static str { //something that lives forever
    return "forward 5
	down 5
	forward 8
	up 3
	down 8
	forward 2";
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parseLine(line: &str) -> Point {
    let (dir, amount) = line.split_once(" ").expect("Must contain whitespaces");
    let amount: i32 = str::parse::<i32>(amount).expect("Second arg must be an integer");
    if dir == "forward" {
        return Point { x: amount, y: 0 };
    } else if dir == "up" {
        return Point { x: 0, y: -amount };
    }

    return Point { x: 0, y: amount };
}

fn main() {
    // error_me(false)?;
    // if error_me(true).is_ok() {}

    // let value = match error_me(false) {
    //     Err(e) => return Err(e),
    //     Ok(v) => v,
    // };

    // return Ok(());

    let result = getInput()
        .lines()
        .map(parseLine)
        .fold(Point { x: 0, y: 0 }, |mut acc, point| {
            acc.x += point.x;
            acc.y += point.y;
            return acc;
        });
    
    print!("{:?}", result);
}
