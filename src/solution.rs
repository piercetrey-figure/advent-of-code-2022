use std::fmt::{Debug, Display};

pub type SolutionPair = (Solution, Solution);

#[derive(PartialEq, Eq)]
pub enum Solution {
    I32(i32),
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::I32(v) => std::fmt::Display::fmt(&v, f),
        }
    }
}

impl Debug for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32(arg0) => f.debug_tuple("I32").field(arg0).finish(),
        }
    }
}

impl From<i32> for Solution {
    fn from(v: i32) -> Self {
        Self::I32(v)
    }
}
