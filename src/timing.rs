use std::time::{self};

pub struct Stopwatch {
    start: time::Instant,
    splits: Vec<(&'static str, time::Instant)>,
}

impl Stopwatch {
    pub fn start() -> Self {
        Self { start: time::Instant::now(), splits: Vec::new() }
    }

    pub fn split(&mut self, label: &'static str) {
        self.splits.push((label, time::Instant::now()));
    }

    pub fn dump(self) -> time::Duration {
        let padding = self.splits.iter().map(|k| k.0.len()).max().unwrap_or(0);
        let total_time =
            self.splits.last().map_or(time::Duration::ZERO, |s| s.1 - self.start);

        println!("Report:");
        let mut last_split = self.start;
        for (label, split) in self.splits {
            let duration = split - last_split;
            let nanos = duration.as_nanos();
            last_split = split;

            print!("{:<pad$} - ", label, pad = padding);
            if nanos >= 3_600_000_000_000 {
                println!("{}h", nanos / 3_600_000_000_000)
            } else if nanos >= 60_000_000_000 {
                println!("{}m", nanos / 60_000_000_000)
            } else if nanos >= 1_000_000_000 {
                println!("{}s", nanos / 1_000_000_000)
            } else if nanos >= 1_000_000 {
                println!("{}ms", nanos / 1_000_000)
            } else if nanos >= 1_000 {
                println!("{}µs", nanos / 1_000)
            } else {
                println!("{}ns", nanos)
            }
        }

        total_time
    }
}
