            "{:?} {}",
            kind,
            String::from_utf8_lossy(&self.bytes[self.idx..self.idx + offset + 1])
        );