use std::{
    error::Error,
    io::{BufRead, stdin},
};

const MAX_LEVEL_DIFF: usize = 3;

fn is_safe(report: Vec<usize>) -> Result<bool, Box<dyn Error>> {
    let is_safe_incr = report
        .iter()
        .is_sorted_by(|x, y| x.gt(y) && x.abs_diff(**y).le(&MAX_LEVEL_DIFF));

    if is_safe_incr {
        return Ok(true);
    }

    let is_safe_decr = report
        .iter()
        .is_sorted_by(|x, y| x.lt(y) && x.abs_diff(**y).le(&MAX_LEVEL_DIFF));

    if is_safe_decr {
        return Ok(true);
    }

    Ok(false)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();

    let mut safe: usize = 0;

    // Reading all input line by line
    let lock_handle = stdin.lock();
    for raw_line in lock_handle.lines() {
        let line = raw_line?;

        if line.is_empty() {
            break;
        }

        let iter = line.split_ascii_whitespace();

        // Parsing and constructing individual reports
        let report: Vec<usize> = iter.map(|x| x.parse::<usize>().unwrap()).collect();

        // Checking report
        if is_safe(report)? {
            safe += 1;
        }
    }

    println!("Answer: {}", safe);

    Ok(())
}
