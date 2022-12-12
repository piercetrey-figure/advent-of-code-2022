use std::fmt::{Debug, Display};

use rug::Integer;

pub type SolutionPair = (Solution, Solution);

#[derive(PartialEq, Eq)]
pub enum Solution {
    I32(i32),
    I64(i64),
    I128(i128),
    U128(u128),
    Integer(Integer),
    String(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::I32(v) => std::fmt::Display::fmt(&v, f),
            Solution::I64(v) => std::fmt::Display::fmt(&v, f),
            Solution::I128(v) => std::fmt::Display::fmt(&v, f),
            Solution::U128(v) => std::fmt::Display::fmt(&v, f),
            Solution::Integer(v) => std::fmt::Display::fmt(&v, f),
            Solution::String(v) => std::fmt::Display::fmt(&v, f),
        }
    }
}

impl Debug for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::I32(arg0) => f.debug_tuple("I32").field(arg0).finish(),
            Solution::I64(arg0) => f.debug_tuple("I64").field(arg0).finish(),
            Solution::I128(arg0) => f.debug_tuple("I128").field(arg0).finish(),
            Solution::U128(arg0) => f.debug_tuple("U128").field(arg0).finish(),
            Solution::Integer(arg0) => f.debug_tuple("Integer").field(arg0).finish(),
            Solution::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
        }
    }
}

impl From<i32> for Solution {
    fn from(v: i32) -> Self {
        Self::I32(v)
    }
}

impl From<i64> for Solution {
    fn from(v: i64) -> Self {
        Self::I64(v)
    }
}

impl From<i128> for Solution {
    fn from(v: i128) -> Self {
        Self::I128(v)
    }
}

impl From<u128> for Solution {
    fn from(v: u128) -> Self {
        Self::U128(v)
    }
}

impl From<Integer> for Solution {
    fn from(v: Integer) -> Self {
        Self::Integer(v)
    }
}

impl From<String> for Solution {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
