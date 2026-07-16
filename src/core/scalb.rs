use std::hint;

pub trait ScalbTen {
    fn scalb10(self, exp: i32) -> Self;
}

impl ScalbTen for f64 {
    fn scalb10(self, exp: i32) -> Self {
        // Could shrink to a few base cases
        const POWERS_OF_TEN: [f64; 32] = [
            1e-22, 1e-21, 1e-20, 1e-19, 1e-18, 1e-17, 1e-16, 1e-15, 1e-14, 1e-13, 1e-12, 1e-11,
            1e-10, 1e-9, 1e-8, 1e-7, 1e-6, 1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1e0, 1e1, 1e2, 1e3, 1e4,
            1e5, 1e6, 1e7, 1e8, 1e9,
        ];

        if (-22..=9).contains(&exp) {
            self * POWERS_OF_TEN[(exp + 22) as usize]
        } else if (-308..=308).contains(&exp) {
            hint::cold_path();
            let mut res = self;
            if exp >= 0 {
                for _ in 0..exp {
                    res *= 10f64;
                }
            } else {
                for _ in 0..-exp {
                    res /= 10f64;
                }
            }
            res
        } else {
            hint::cold_path();
            if exp > 0 { if self > 0f64 { f64::INFINITY } else { f64::NEG_INFINITY } } else { 0f64 }
        }
    }
}
