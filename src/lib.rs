use core::fmt;
use std::str::FromStr;

use conversion::jdn_to_eth;
use time::{self, error};

mod conversion;
mod validator;

// Maybe change the name of  this struct to `Werh` which translates to month in `ge'ez`, to avoid
// conflict with `time::Month`.
/// Months of the Ethiopian year.
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Month {
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

impl TryFrom<u8> for Month {
    type Error = ();

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
            _ => Err(()),
        }
    }
}


impl FromStr for Month {
    type Err = ();

    fn from_str(month_name: &str) -> Result<Self, Self::Err> {
        match month_name {
            "Meskerem" => Ok(Month::Meskerem),
            "Tikimit" => Ok(Month::Tikimit),
            "Hedar" => Ok(Month::Hedar),
            "Tahasass" => Ok(Month::Tahasass),
            "Yekatit" => Ok(Month::Yekatit),
            "Megabit" => Ok(Month::Megabit),
            "Miyazia" => Ok(Month::Miyazia),
            "Sene" => Ok(Month::Sene),
            "Hamle" => Ok(Month::Hamle),
            "Nehase" => Ok(Month::Nehase),
            "Puagme" => Ok(Month::Puagme),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Month {
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

/// An Ethiopian Date.
#[derive(Debug, PartialEq, Clone)]
pub struct Zemen {
    year: i32,
    month: Month,
    day: u8,
}

impl fmt::Display for Zemen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}/{}", self.month, self.day, self.year)
    }
}

impl Zemen {
    fn new(year: i32, month: u8, day: u8) -> Result<Self, error::ComponentRange> {
        // TODO: validate Ethiopian date
        let month = Month::try_from(month).expect("must between 1 and 13");
        Ok(Zemen { year, month, day })
    }

    /// Get the year.
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// let qen: Zemen = Zemen::from_eth_cal(2000, 1, 1).unwrap();
    /// assert_eq!(qen.year(), 2000);
    /// ```
    pub fn year(&self) -> i32 { self.year }

    /// Get the month.
    ///
    /// ```rust
    /// # use zemen::{Zemen, Month};
    ///
    /// let qen: Zemen = Zemen::from_eth_cal(2000, 1, 1).unwrap();
    /// assert_eq!(qen.month(), Month::Meskerem);
    /// ```
    pub fn month(&self) -> Month {
        self.month
    }

    /// Get the day of the month.
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// let qen: Zemen = Zemen::from_eth_cal(2000, 1, 30).unwrap();
    /// assert_eq!(qen.day(), 30);
    /// ```
    pub fn day(&self) -> u8 { self.day }

    /// returns the current date in Ethiopian date.
    ///
    /// ```rust
    /// # use time;
    /// # use zemen::Zemen;
    ///
    /// let today: time::Date = time::OffsetDateTime::now_utc().date();
    /// let zare: Zemen = Zemen::today();
    ///
    /// assert_eq!(today, zare.to_gre());
    /// ```
    pub fn today() -> Self {
        let today = time::OffsetDateTime::now_utc().date();
        conversion::gre_to_eth(
            today.year(),
            today.month() as u8,
            today.day(),
        ).expect("Since `today` is valid we know this won't fail.")
    }

    /// Takes a gregorian calendar, converts into Ethiopian calendar and
    /// returns a `Zemen` instance.
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use time::{Date, Month};
    ///
    /// let date: Date = Date::from_calendar_date(2000, Month::January, 1).unwrap();
    /// let qen: Zemen = Zemen::from_gre_cal(2000, 1, 1).unwrap();
    ///
    /// assert_eq!(date, qen.to_gre());
    /// ```
    pub fn from_gre_cal(year: i32, month: u8, day: u8) -> Result<Self, error::ComponentRange> {
        conversion::gre_to_eth(
            year, month, day
        )
    }

    /// Builds a new `Zemen` instance from a borrowed `time::Date`.
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use time::{Date, Month};
    ///
    /// let date: Date = Date::from_calendar_date(2000, Month::January, 1).unwrap();
    /// let qen: Zemen = Zemen::from_gre_date(&date);
    /// assert_eq!(date, qen.to_gre());
    /// ```
    pub fn from_gre_date(gc_date: &time::Date) -> Self {
        conversion::gre_to_eth(
            gc_date.year(),
            gc_date.month() as u8,
            gc_date.day(),
        ).expect("Since `gc_date` is a valid date the returned date will also be valid")
    }

    /// Create an Ethiopian date from it's number representations
    ///
    /// ```rust
    /// # use zemen::{Zemen, Month};
    /// let qen = Zemen::from_eth_cal(1992, 4, 22).unwrap();
    ///
    /// assert_eq!(qen.year(), 1992);
    /// assert_eq!(qen.month(), Month::Tahasass);
    /// assert_eq!(qen.day(), 22);
    /// ```
    pub fn from_eth_cal(year: i32, month: u8, day: u8) -> Result<Self, error::ComponentRange> {
        Ok(Self::new(year, month, day)?)
    }

    /// Convertes the current Ethiopian date into `time::Date`.
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use time::{Date, Month};
    /// let qen: Zemen = Zemen::from_eth_cal(1992, 4, 22).unwrap();
    /// let date: Date = Date::from_calendar_date(2000, Month::January, 1).unwrap();
    ///
    /// assert_eq!(date, qen.to_gre());
    /// ```
    pub fn to_gre(&self) -> time::Date {
        conversion::eth_to_gre(
            self.year,
            self.month as u8,
            self.day
        ).expect("Since we are able to create an instance of `Zemen` in the beginning. we dont need to return errors")
    }

    /// Create an Ethiopian date from Julian day number.
    ///
    /// ```rust
    /// # use zemen::Zemen;
    ///
    /// assert_eq!(Zemen::from_jdn(2_451_545), Zemen::from_eth_cal(1992, 4, 22));
    /// assert_eq!(Zemen::from_jdn(2_458_485), Zemen::from_eth_cal(2011, 4, 23));
    /// assert_eq!(Zemen::from_jdn(2_458_849), Zemen::from_eth_cal(2012, 4, 21));
    /// ```
    pub fn from_jdn(jdn: i32) -> Result<Self, error::ComponentRange> {
        let (year, month, day) = jdn_to_eth(jdn);
        Ok(Self::from_eth_cal(year, month, day)?)
    }

    /// Get the Julian day number for the Ethiopian date.
    ///
    /// ```rust
    /// # use zemen::Zemen;
    ///
    /// assert_eq!(Zemen::from_eth_cal(1992, 4, 22).unwrap().to_jdn(), 2_451_545);
    /// assert_eq!(Zemen::from_eth_cal(2011, 4, 23).unwrap().to_jdn(), 2_458_485);
    /// assert_eq!(Zemen::from_eth_cal(2012, 4, 21).unwrap().to_jdn(), 2_458_849);
    /// ```
    pub fn to_jdn(&self) -> i32 {
        conversion::eth_to_jdn(
            self.year, self.month as i32, self.day as i32
        )
    }
}

#[cfg(test)]
mod tests {
    use time::error;
    use time;

    use crate::Zemen;
    use crate::Month;

    #[test]
    fn test_get_current_date_and_convert_to_gre() -> Result<(), error::ComponentRange> {
        let zare = Zemen::today();
        println!("Zare (initiated): {}", zare);

        let converted_date = zare.to_gre();
        println!("Today (converted): {}", converted_date);

        let converted_zare = Zemen::from_gre_date(&converted_date);
        println!("Zare (converted): {}", converted_zare);

        Ok(())
    }

    #[test]
    fn test_month_creating_and_basic_parsing() -> Result<(), ()> {
        let wer_num = Month::try_from(13)?;
        let wer_enum_pag = Month::Puagme;

        assert_eq!(wer_enum_pag, wer_num);

        let wer_enum_mesk = Month::Meskerem;
        let wer_str: Month = "Meskerem".parse()?;

        assert_eq!(wer_enum_mesk, wer_str);

        println!("wer_num: {}", wer_num);
        println!("wer_str: {}", wer_str);

        Ok(())
    }
}
