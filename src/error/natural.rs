use std::fmt::Display;

use arrayvec::ArrayVec;

pub trait Natural {
    fn join_natural(&self, sep: &str) -> String;
}

impl<T: Display> Natural for [T] {
    fn join_natural(&self, sep: &str) -> String {
        match self.len() {
            0 => String::new(),
            1 => self[0].to_string(),
            2 => format!("{} {} {}", self[0], sep, self[1]),
            _ => {
                let pre_conj = self[0..self.len() - 1]
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{}, {} {}", pre_conj, sep, self[self.len() - 1])
            }
        }
    }
}

impl<T: Display> Natural for Vec<T> {
    fn join_natural(&self, sep: &str) -> String {
        self.as_slice().join_natural(sep)
    }
}

impl<T: Display, const CAP: usize> Natural for ArrayVec<T, CAP> {
    fn join_natural(&self, sep: &str) -> String {
        self.as_slice().join_natural(sep)
    }
}
