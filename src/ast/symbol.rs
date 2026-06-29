use std::fmt;

pub trait Symbol: fmt::Debug + Copy + Clone {
    fn name(&self) -> &str;
    fn as_str(&self) -> &str;
}
