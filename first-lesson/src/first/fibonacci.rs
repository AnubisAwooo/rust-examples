struct Fibonacci {
    a: u64,
    b: u64,
    current: usize,
    max: usize,
}

impl Fibonacci {
    fn new(max: usize) -> Self {
        Self {
            a: 0,
            b: 0,
            current: 0,
            max,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.max {
            return None;
        }

        if self.current == 0 {
            self.a = 1;
            self.b = 1;
        } else {
            let sum = self.a + self.b;
            self.a = self.b;
            self.b = sum;
        }

        self.current += 1;
        Some(self.a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let fibonacci = Fibonacci::new(10);

        for i in fibonacci {
            println!("{}", i);
        }
    }
}
