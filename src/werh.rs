//! Todo: Documentations
use std::fmt;
use std::str::FromStr;

use crate::error;

type Result<T> = std::result::Result<T, crate::error::Error>;

/// Months of the Ethiopian year. `Werh` means month in Ge'ez.
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Werh {
    Meskerem = 1,
    Tikimit = 2,
    Hedar = 3,
    Tahasass = 4,
    Tir = 5,
    Yekatit = 6,
    Megabit = 7,
    Miyazia = 8,
    Ginbot = 9,
    Sene = 10,
    Hamle = 11,
    Nehase = 12,
    Puagme = 13,
}

impl Werh {
    /// Get the next `Werh`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Werh;
    /// assert_eq!(Werh::Meskerem.next(), Werh::Tikimit);
    /// assert_eq!(Werh::Tikimit.next(), Werh::Hedar);
    /// ```
    pub fn next(self) -> Self {
        match self {
            Self::Meskerem => Self::Tikimit,
            Self::Tikimit => Self::Hedar,
            Self::Hedar => Self::Tahasass,
            Self::Tahasass => Self::Tir,
            Self::Tir => Self::Yekatit,
            Self::Yekatit => Self::Megabit,
            Self::Megabit => Self::Miyazia,
            Self::Miyazia => Self::Ginbot,
            Self::Ginbot => Self::Sene,
            Self::Sene => Self::Hamle,
            Self::Hamle => Self::Nehase,
            Self::Nehase => Self::Puagme,
            Self::Puagme => Self::Meskerem,
        }
    }

    /// Get the previous `Werh`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Werh;
    /// assert_eq!(Werh::Meskerem.previous(), Werh::Puagme);
    /// assert_eq!(Werh::Tikimit.previous(), Werh::Meskerem);
    /// ```
    pub fn previous(self) -> Self {
        match self {
            Self::Meskerem => Self::Puagme,
            Self::Tikimit => Self::Meskerem,
            Self::Hedar => Self::Tikimit,
            Self::Tahasass => Self::Hedar,
            Self::Tir => Self::Tahasass,
            Self::Yekatit => Self::Tir,
            Self::Megabit => Self::Yekatit,
            Self::Miyazia => Self::Megabit,
            Self::Ginbot => Self::Miyazia,
            Self::Sene => Self::Ginbot,
            Self::Hamle => Self::Sene,
            Self::Nehase => Self::Hamle,
            Self::Puagme => Self::Nehase,
        }
    }

    /// Get the short name of the month.
    ///
    /// # Examples
    /// ```rust
    /// # use zemen::Werh;
    /// assert_eq!(Werh::Meskerem.short_name(), "መስከ");
    /// assert_eq!(Werh::Tir.short_name(), "ጥር");
    /// ```
    pub fn short_name(&self) -> String {
        self.to_string().chars().take(3).collect()
    }
}

impl TryFrom<u8> for Werh {
    type Error = error::Error;

    /// Converts an `u8` to it's `Werh` equivalent
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::{Werh, error};
    /// let wer = Werh::try_from(9)?;
    ///
    /// assert_eq!(Werh::Ginbot, wer);
    /// # Ok::<(), error::Error>(())
    /// ```
    fn try_from(num: u8) -> Result<Self> {
        match num {
            1 => Ok(Self::Meskerem),
            2 => Ok(Self::Tikimit),
            3 => Ok(Self::Hedar),
            4 => Ok(Self::Tahasass),
            5 => Ok(Self::Tir),
            6 => Ok(Self::Yekatit),
            7 => Ok(Self::Megabit),
            8 => Ok(Self::Miyazia),
            9 => Ok(Self::Ginbot),
            10 => Ok(Self::Sene),
            11 => Ok(Self::Hamle),
            12 => Ok(Self::Nehase),
            13 => Ok(Self::Puagme),
            oth => Err(error::Error::InvalidRange {
                name: "Werh",
                given: oth as i32,
                max: 13,
                min: 1,
            }),
        }
    }
}

impl FromStr for Werh {
    type Err = error::Error;

    /// Parses the given string to `Werh`. It's case insensitive.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::{Werh, error};
    /// let mesk: Werh = "meskerem".parse()?;
    /// let tikm = "TiKimiT".parse::<Werh>()?;
    ///
    /// assert_eq!(Werh::Meskerem, mesk);
    /// assert_eq!(Werh::Tikimit, tikm);
    /// # Ok::<(), error::Error>(())
    /// ```
    fn from_str(month_name: &str) -> Result<Self> {
        let lower = month_name.to_lowercase();
        match lower.as_str() {
            "meskerem" | "መስከረም" => Ok(Werh::Meskerem),
            "tikimit" | "ጥቅምት" => Ok(Werh::Tikimit),
            "hedar" | "ኅዳር" => Ok(Werh::Hedar),
            "tahasass" | "ታኅሣሥ" => Ok(Werh::Tahasass),
            "tir" | "ጥር" => Ok(Werh::Tir),
            "yekatit" | "የካቲት" => Ok(Werh::Yekatit),
            "megabit" | "መጋቢት" => Ok(Werh::Megabit),
            "miyazia" | "ሚያዝያ" => Ok(Werh::Miyazia),
            "ginbot" | "ግንቦት" => Ok(Werh::Ginbot),
            "sene" | "ሰኔ" => Ok(Werh::Sene),
            "hamle" | "ሐምሌ" => Ok(Werh::Hamle),
            "nehase" | "ነሐሴ" => Ok(Werh::Nehase),
            "puagme" | "ጳጉሜ" => Ok(Werh::Puagme),
            _ => Err(error::Error::InvalidVariant("Werh", lower)),
        }
    }
}

impl fmt::Display for Werh {
    /// Formats the current `Werh`'s name into it's amharic format
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Werh;
    /// let wer = Werh::Meskerem;
    ///
    /// assert_eq!("መስከረም", wer.to_string())
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Meskerem => "መስከረም",
            Self::Tikimit => "ጥቅምት",
            Self::Hedar => "ኅዳር",
            Self::Tahasass => "ታኅሣሥ",
            Self::Tir => "ጥር",
            Self::Yekatit => "የካቲት",
            Self::Megabit => "መጋቢት",
            Self::Miyazia => "ሚያዝያ",
            Self::Ginbot => "ግንቦት",
            Self::Sene => "ሰኔ",
            Self::Hamle => "ሐምሌ",
            Self::Nehase => "ነሐሴ",
            Self::Puagme => "ጳጉሜ",
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn test_werh_from_number() {
        let _w = Werh::try_from(18).unwrap();
    }

    #[test]
    fn test_month_from_english_text() -> Result<()> {
        let amh_month_name = [
            "መስከረም",
            "ጥቅምት",
            "ኅዳር",
            "ታኅሣሥ",
            "ጥር",
            "የካቲት",
            "መጋቢት",
            "ሚያዝያ",
            "ግንቦት",
            "ሰኔ",
            "ሐምሌ",
            "ነሐሴ",
            "ጳጉሜ",
        ];
        let eng_month_name = [
            "meskerem", "tikimit", "hedar", "tahasass", "tir", "yekatit", "megabit", "miyazia",
            "ginbot", "sene", "hamle", "nehase", "puagme",
        ];

        for (_month_num, (awn, ewn)) in amh_month_name.iter().zip(eng_month_name).enumerate() {
            let month_eng = Werh::from_str(awn)?;
            let month_amh = Werh::from_str(ewn)?;

            assert_eq!(month_amh, month_eng);
        }

        Ok(())
    }
}
