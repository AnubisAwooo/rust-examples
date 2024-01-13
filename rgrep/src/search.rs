use colored::*;
use regex::Regex;

use crate::read::Source;

pub fn search(source: Source, word: String) -> (String, Vec<String>) {
    let re = Regex::new(&word).unwrap();
    (
        source.source,
        source
            .lines
            .enumerate()
            .map(|(i, line)| {
                let row = i + 1;
                if let Some(cap) = re.captures(&line) {
                    let iter = cap.iter();
                    return iter
                        .filter_map(|m| {
                            if let Some(m) = m {
                                let start = m.start();
                                let end = m.end();
                                return Some(format!(
                                    "{row: >6}:{start: <3} {}{}{}",
                                    line.split_at(start).0,
                                    line.split_at(start).1.split_at(end - start).0.red(),
                                    line.split_at(end).1
                                ));
                            }
                            None
                        })
                        .collect();
                }
                vec![]
            })
            .flat_map(|f| f)
            .collect(),
    )
}
