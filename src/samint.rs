//! Todo: Documentations

use crate::error;
use std::{fmt, str::FromStr};

///  Weekdays of the Ethiopian calendar, `Samint` directly translates to week, but in our case it
///  is enough
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Samint {
    Ihud = 0,
    Senyo = 1,
    Makisenyo = 2,
    Irob = 3,
    Hamus = 4,
    Arb = 5,
    Kidame = 6,
}

impl Samint {
    /// Get the next day in the week.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Samint;
    /// let qen = Samint::Ihud;
    ///
    /// assert_eq!(qen.next(), Samint::Senyo);
    /// ```
    pub fn next(self) -> Self {
        match self {
            Samint::Ihud => Samint::Senyo,
            Samint::Senyo => Samint::Makisenyo,
            Samint::Makisenyo => Samint::Irob,
            Samint::Irob => Samint::Hamus,
            Samint::Hamus => Samint::Arb,
            Samint::Arb => Samint::Kidame,
            Samint::Kidame => Samint::Ihud,
        }
    }

    /// Get the previous day in the week.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Samint;
    /// let qen = Samint::Ihud;
    ///
    /// assert_eq!(qen.previous(), Samint::Kidame);
    /// ```
    pub fn previous(self) -> Self {
        match self {
            Samint::Ihud => Samint::Kidame,
            Samint::Senyo => Samint::Ihud,
            Samint::Makisenyo => Samint::Senyo,
            Samint::Irob => Samint::Makisenyo,
            Samint::Hamus => Samint::Irob,
            Samint::Arb => Samint::Hamus,
            Samint::Kidame => Samint::Arb,
        }
    }

    /// Get short name of the Weekday
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Samint;
    /// assert_eq!(Samint::Ihud.short_name(), "እሑድ")
    /// ```
    pub fn short_name(&self) -> String {
        self.to_string().chars().take(3).collect()
    }
}

impl TryFrom<u8> for Samint {
    type Error = error::Error;

    /// Given a `u8` should try to return the equivalent `Samint`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::{error, Samint};
    /// # fn main() -> Result<(), error::Error> {
    /// let elet = Samint::try_from(0)?;
    ///
    /// assert_eq!(Samint::Ihud, elet);
    /// # Ok(())
    /// # }
    /// ```
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ihud),
            1 => Ok(Self::Senyo),
            2 => Ok(Self::Makisenyo),
            3 => Ok(Self::Irob),
            4 => Ok(Self::Hamus),
            5 => Ok(Self::Arb),
            6 => Ok(Self::Kidame),
            oth => Err(error::Error::InvalidRange {
                name: "Samint",
                given: oth as i32,
                min: 0,
                max: 6,
            }),
        }
    }
}
impl fmt::Display for Samint {
    /// Formats a `Samint` into amharic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Samint;
    /// let qen = Samint::Ihud;
    ///
    /// assert_eq!("እሑድ", qen.to_string())
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Ihud => "እሑድ",
            Self::Senyo => "ሰኞ",
            Self::Makisenyo => "ማክሰኞ",
            Self::Irob => "ረቡዕ",
            Self::Hamus => "ሐሙስ",
            Self::Arb => "ዓርብ",
            Self::Kidame => "ቅዳሜ",
        })
    }
}

impl FromStr for Samint {
    type Err = error::Error;

    /// Given a string `s` it will try to parse it into `Samint` case-insensitively.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::{Samint, error};
    /// # fn main() -> Result<(), error::Error> {
    /// let qen: Samint = "ihuD".parse()?;
    ///
    /// assert_eq!(Samint::Ihud, qen);
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ihud" => Ok(Samint::Ihud),
            "senyo" => Ok(Samint::Senyo),
            "makisenyo" => Ok(Samint::Makisenyo),
            "irob" => Ok(Samint::Irob),
            "hamus" => Ok(Samint::Hamus),
            "arb" => Ok(Samint::Arb),
            "kidame" => Ok(Samint::Kidame),
            _ => Err(error::Error::InvalidVariant("Samint")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samint_from_u8() {
        let elet = Samint::try_from(6);
        match elet {
            Ok(o) => println!("o: {}", o),
            Err(e) => panic!("should succeed, failded with: {}", e),
        }

        assert_eq!(elet.unwrap(), Samint::Kidame);

        let elet = Samint::try_from(8);
        match elet {
            Ok(o) => panic!("should fail, succeeded with: {}", o),
            Err(e) => println!("e: {}", e),
        }
    }

    #[test]
    fn test_short_weekday_names() {
        for e in 0..=6 {
            let elet = Samint::try_from(e).expect("should be between 0 and 6");

            println!("Short day name: {}", elet.short_name());
        }
    }
}
