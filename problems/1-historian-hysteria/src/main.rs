use std::{
    error::Error,
    io::{BufRead, stdin},
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();

    let mut col_a: Vec<usize> = Vec::new();
    let mut col_b: Vec<usize> = Vec::new();

    // Reading all input line by line
    let lock_handle = stdin.lock();
    for raw_line in lock_handle.lines() {
        let line = raw_line?;

        if line.is_empty() {
            break;
        }

        let mut iter = line.split_ascii_whitespace();

        // Parsing and pushing to our vectors
        let a = iter.next().unwrap().parse::<usize>()?;
        let b = iter.next().unwrap().parse::<usize>()?;
        col_a.push(a);
        col_a.sort_unstable();
        col_b.push(b);
        col_b.sort_unstable();
    }

    let mut ans = 0;

    // pair up elements from each col and sum
    col_a.iter().zip(col_b.iter()).for_each(|(x, y)| {
        ans += x.abs_diff(*y);
    });

    println!("Answer: {}", ans);

    Ok(())
}
