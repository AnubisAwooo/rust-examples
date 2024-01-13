pub struct Source {
    pub source: String,
    pub lines: Box<dyn Iterator<Item = String> + Send + 'static>,
}
pub trait ReadLine {
    fn lines(&self) -> Source;
}

struct FileRead {
    file: String,
}

impl ReadLine for FileRead {
    fn lines(&self) -> Source {
        use std::io::BufRead;
        Source {
            source: self.file.clone(),
            lines: Box::new(
                std::io::BufReader::new(
                    std::fs::File::open(&self.file)
                        .expect(&format!("can not find file: {}", self.file)),
                )
                .lines()
                .into_iter()
                .map(|line| line.expect("read line failed")),
            ),
        }
    }
}

struct StdinRead;

impl Iterator for StdinRead {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("read stdin failed");
        while line.ends_with('\n') {
            line.pop();
        }
        if line == "exit" {
            None
        } else {
            Some(line)
        }
    }
}

impl ReadLine for StdinRead {
    fn lines(&self) -> Source {
        Source {
            source: "stdin".into(),
            lines: Box::new(StdinRead.into_iter()),
        }
    }
}

pub fn parse_read_lines(files: Vec<String>) -> Vec<Source> {
    if files.is_empty() || files.iter().any(|f| f == "-") {
        return vec![StdinRead.lines()];
    }
    files
        .into_iter()
        .map(|file| FileRead { file }.lines())
        .collect::<Vec<_>>()
}
