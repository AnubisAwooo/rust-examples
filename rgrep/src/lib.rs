use std::{
    io::{BufRead, Write},
    ops::Range,
};

use colored::Colorize;
use regex::Regex;

pub mod args;

pub mod read;

pub mod search;

pub type ExecuteFn = fn(&Regex, &mut dyn BufRead, &mut dyn Write) -> usize;

pub fn default_execute(regex: &Regex, reader: &mut dyn BufRead, writer: &mut dyn Write) -> usize {
    let mut count = 0;

    writer
        .write(
            reader
                .lines()
                .enumerate()
                .filter_map(|(no, line)| {
                    line.ok().and_then(|line| {
                        regex.find(&line).map(|m| {
                            count += 1;
                            let Range { start, end } = m.range();
                            let prefix = &line[..start];
                            format!(
                                "{: >6}:{: <3} {}{}{}",
                                (no + 1).to_string().blue(),
                                (prefix.chars().count() + 1).to_string().cyan(),
                                prefix,
                                &line[start..end].red(),
                                &line[end..]
                            )
                        })
                    })
                })
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        )
        .unwrap();

    count
}
