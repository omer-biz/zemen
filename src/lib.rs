
mod conversion;

#[derive(Debug, PartialEq)]
pub struct Zemen {
    year: i32,
    month: u8,
    day: u8,
}

impl Zemen {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Zemen { year, month, day }
    }

    pub fn year(&self) -> i32 { self.year }
    pub fn month(&self) -> u8 { self. month }
    pub fn day(&self) -> u8 { self.day }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
