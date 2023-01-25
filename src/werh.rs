use std::fmt;
use std::str::FromStr;

use crate::error;

// Maybe change the name of  this struct to `Werh` which translates to month in `ge'ez`, to avoid
// conflict with `time::Month`.
/// Months of the Ethiopian year.
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
    Puagme = 13
}

impl TryFrom<u8> for Werh {
    type Error = error::InvalidDateRangeError;

    fn try_from(num: u8) -> Result<Self, Self::Error> {
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
            oth => Err(error::InvalidDateRangeError { 
                name: "Month", given: oth as i32, max: 13, min: 1 
            }),
        }
    }
}


impl FromStr for Werh {
    type Err = error::InvalidVariant;

    fn from_str(month_name: &str) -> Result<Self, Self::Err> {
        match month_name {
            "Meskerem" => Ok(Werh::Meskerem),
            "Tikimit" => Ok(Werh::Tikimit),
            "Hedar" => Ok(Werh::Hedar),
            "Tahasass" => Ok(Werh::Tahasass),
            "Yekatit" => Ok(Werh::Yekatit),
            "Megabit" => Ok(Werh::Megabit),
            "Miyazia" => Ok(Werh::Miyazia),
            "Sene" => Ok(Werh::Sene),
            "Hamle" => Ok(Werh::Hamle),
            "Nehase" => Ok(Werh::Nehase),
            "Puagme" => Ok(Werh::Puagme),
            _ => Err(error::InvalidVariant { name: "Month" }),
        }
    }
}

impl fmt::Display for Werh {
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
