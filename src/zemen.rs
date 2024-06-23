//! Todo: Documentations.

type Result<T> = std::result::Result<T, crate::error::Error>;

use crate::{conversion, error, formatting, validator, Samint, Werh};
use std::{fmt, ops::Add};

/// An Ethiopian Date.
#[derive(PartialEq, Clone)]
pub struct Zemen {
    // the first 9 bits will store the ordinal day
    // the rest is for the year.
    // 0000 0000 0000 0000 0000 000 0 0000 0000
    //           year               |ordianl day
    ordinal_date: i32,
}

impl fmt::Display for Zemen {
    /// Formats the current date into `YY-MM-DD`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::{Zemen, Werh, error};
    /// let qen = Zemen::from_eth_cal(2000, Werh::Meskerem, 1)?;
    ///
    /// assert_eq!(qen.to_string(), "2000-01-01");
    /// # Ok::<(), error::Error>(())
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}",
            self.year(),
            self.month() as u8,
            self.day()
        )
    }
}

impl fmt::Debug for Zemen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}",
            self.year(),
            self.month() as u8,
            self.day()
        )
    }
}

#[cfg(feature = "time")]
impl From<&Zemen> for time::Date {
    /// Converts `zemen::Zemen`, which is in Ethiopian format,
    /// to it's Gregorain format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::error;
    /// # use time::Date;
    /// # use time::Month;
    /// # use zemen::Werh;
    /// let qen = Zemen::from_eth_cal(1992, Werh::Tahasass, 22)?;
    /// let day = Date::from(&qen);
    ///
    /// assert_eq!(2000, day.year());
    /// assert_eq!(Month::January, day.month());
    /// assert_eq!(1, day.day());
    /// # Ok::<(), error::Error>(())
    /// ```
    fn from(value: &Zemen) -> Self {
        // Zemen::from_eth_date(value)
        conversion::eth_to_gre(value.year(), value.month() as u8, value.day())
            .expect("From already a validated date")
    }
}

#[cfg(feature = "time")]
impl From<&time::Date> for Zemen {
    /// Converts `time::Date`, which is in Greogrian format,
    /// to it's Ethiopian format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::error;
    /// # use zemen::Werh;
    /// # use time::Date;
    /// # use time::Month;
    ///
    /// let day = Date::from_calendar_date(2000, Month::January, 1)?;
    /// let qen = Zemen::from(&day);
    ///
    /// assert_eq!(1992, qen.year());
    /// assert_eq!(Werh::Tahasass, qen.month());
    /// assert_eq!(22, qen.day());
    /// # Ok::<(), error::Error>(())
    /// ```
    fn from(value: &time::Date) -> Self {
        Self::from_date(value)
    }
}

impl Add<i32> for Zemen {
    type Output = Zemen;

    /// Adding a number to a `Zemen` instance will advance it by the number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::error;
    /// # use zemen::Werh;
    /// let qen = Zemen::from_eth_cal(2003, Werh::Puagme, 1)?;
    /// let qen = qen + 6;
    ///
    /// assert_eq!(qen, Zemen::from_eth_cal(2004, Werh::Meskerem, 1)?);
    /// # Ok::<(), error::Error>(())
    /// ```
    fn add(self, days: i32) -> Self::Output {
        Zemen::from_jdn(self.to_jdn() + days).expect("`to_jdn` gives us a valid jdn date")
    }
}

impl Zemen {
    pub(crate) fn new(year: i32, month: u8, day: u8) -> Result<Self> {
        validator::is_valid_date(year, month, day)?;
        Self::from_ordinal_date(year, conversion::to_ordinal(month as i32, day as i32) as _)
    }

    /// Attempt to create a `Zemen` from the year and day number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::Werh;
    /// # use zemen::error;
    /// let qen = Zemen::from_ordinal_date(2000, 62)?;
    /// assert_eq!(qen, Zemen::from_eth_cal(2000, Werh::Hedar, 2)?);
    /// # Ok::<(), error::Error>(())
    /// ```
    pub fn from_ordinal_date(year: i32, ordinal: u16) -> Result<Self> {
        error::is_in_range(
            ordinal as _,
            1,
            validator::days_in_year(year) as _,
            "ordinal",
        )?;

        Ok(Zemen {
            ordinal_date: (year << 9) | ordinal as i32,
        })
    }

    /// Get the year.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::Werh;
    /// # use zemen::error;
    /// let qen: Zemen = Zemen::from_eth_cal(2000, Werh::Meskerem, 1)?;
    /// assert_eq!(qen.year(), 2000);
    /// # Ok::<(), error::Error>(())
    /// ```
    pub fn year(&self) -> i32 {
        self.ordinal_date >> 9
    }

    /// Get the month.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::{Zemen, Werh, error};
    ///
    /// let qen: Zemen = Zemen::from_eth_cal(2000, Werh::Meskerem, 1)?;
    /// assert_eq!(qen.month(), Werh::Meskerem);
    /// # Ok::<(), error::Error>(())
    /// ```
    pub fn month(&self) -> Werh {
        let (month, _) = conversion::from_ordinal(self.ordinal() as _);
        Werh::try_from(month as u8).expect("validated by new")
    }

    /// Get the day of the month.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::Werh;
    /// # use zemen::error;
    ///
    /// let qen: Zemen = Zemen::from_eth_cal(2000, Werh::Meskerem, 30)?;
    /// assert_eq!(qen.day(), 30);
    /// # Ok::<(), error::Error>(())
    /// ```
    pub fn day(&self) -> u8 {
        let (_, day) = conversion::from_ordinal(self.ordinal() as _);
        day as u8
    }

    /// returns the current date in Ethiopian date.
    ///
    /// # Examples
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
    #[cfg(feature = "time")]
    pub fn today() -> Self {
        let today = time::OffsetDateTime::now_utc().date();
        conversion::gre_to_eth(today.year(), today.month() as u8, today.day())
            .expect("Since `today` is valid conversion won't fail")
    }

    /// Converts `&time::Date` (Gregorian date) to `zemen::Zemen` (Ethiopian date)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::error;
    /// # use time::{Date, Month};
    ///
    /// let date: Date = Date::from_calendar_date(2000, Month::January, 1)?;
    /// let qen: Zemen = Zemen::from_date(&date);
    /// assert_eq!(date, qen.to_gre());
    /// # Ok::<(), error::Error>(())
    /// ```
    #[cfg(feature = "time")]
    pub fn from_date(gc_date: &time::Date) -> Self {
        conversion::gre_to_eth(gc_date.year(), gc_date.month() as u8, gc_date.day())
            .expect("since `gc_date` is a valid date the returned date will also be valid")
    }

    /// Create an Ethiopian date from it's number representations
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::{Zemen, Werh, error};
    /// let qen = Zemen::from_eth_cal(1992, Werh::Tahasass, 22)?;
    ///
    /// assert_eq!(qen.year(), 1992);
    /// assert_eq!(qen.month(), Werh::Tahasass);
    /// assert_eq!(qen.day(), 22);
    /// # Ok::<(), error::Error>(())
    ///
    /// ```
    pub fn from_eth_cal(year: i32, month: Werh, day: u8) -> Result<Self> {
        Self::new(year, month as u8, day)
    }

    /// Create an Ethiopian date from Gregorian date
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::{Zemen, error, Werh};
    ///
    /// let qen: Zemen = Zemen::from_gre_cal(2000, 1, 1)?;
    /// assert_eq!(qen.year(), 1992);
    /// assert_eq!(qen.month(), Werh::Tahasass);
    /// assert_eq!(qen.day(), 22);
    /// # Ok::<(), error::Error>(())
    /// ```
    #[cfg(feature = "time")]
    pub fn from_gre_cal(year: i32, month: u8, day: u8) -> Result<Self> {
        conversion::gre_to_eth(year, month, day)
    }

    /// Convertes the current Ethiopian date in `time::Date`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::error;
    /// # use time::{Date, self};
    /// let qen: Zemen = Zemen::from_eth_cal(1992, zemen::Werh::Tahasass, 22)?;
    /// let date: Date = Date::from_calendar_date(2000, time::Month::January, 1)?;
    ///
    /// assert_eq!(date, qen.to_gre());
    /// # Ok::<(), error::Error>(())
    /// ```
    #[cfg(feature = "time")]
    pub fn to_gre(&self) -> time::Date {
        conversion::eth_to_gre(
            self.year(),
            self.month() as u8,
            self.day()
        ).expect("Since we are able to create an instance of `Zemen` in the beginning. we dont need to return errors")
    }

    /// Create an Ethiopian date from Julian day number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::Werh;
    /// # use zemen::error;
    ///
    /// assert_eq!(Zemen::from_jdn(2_451_545)?, Zemen::from_eth_cal(1992, Werh::Tahasass, 22)?);
    /// assert_eq!(Zemen::from_jdn(2_458_485)?, Zemen::from_eth_cal(2011, Werh::Tahasass, 23)?);
    /// assert_eq!(Zemen::from_jdn(2_458_849)?, Zemen::from_eth_cal(2012, Werh::Tahasass, 21)?);
    /// # Ok::<(), error::Error>(())
    /// ```
    pub fn from_jdn(jdn: i32) -> Result<Self> {
        let (year, month, day) = conversion::jdn_to_eth(jdn);
        let month: Werh = Werh::try_from(month)?;

        Self::from_eth_cal(year, month, day)
    }

    /// Get the Julian day number for the Ethiopian date.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::Werh;
    /// # use zemen::error;
    ///
    /// assert_eq!(Zemen::from_eth_cal(1992, Werh::Tahasass, 22)?.to_jdn(), 2_451_545);
    /// assert_eq!(Zemen::from_eth_cal(2011, Werh::Tahasass, 23)?.to_jdn(), 2_458_485);
    /// assert_eq!(Zemen::from_eth_cal(2012, Werh::Tahasass, 21)?.to_jdn(), 2_458_849);
    /// # Ok::<(), error::Error>(())
    /// ```
    pub fn to_jdn(&self) -> i32 {
        conversion::eth_to_jdn(self.year(), self.month() as i32, self.day() as i32)
    }

    /// Get the weekday.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::{Werh, Zemen, error, Samint};
    /// let qen = Zemen::from_eth_cal(1992, Werh::Tahasass, 22)?;
    /// // because `time::Weekday` starts with `Monday` we have to increment by one
    /// assert_eq!(qen.weekday(),  Samint::Kidame);
    /// # Ok::<(), error::Error>(())
    /// ```
    pub fn weekday(&self) -> Samint {
        let weekday = (self.to_jdn() + 1) % 7;
        Samint::try_from(weekday as u8)
            .expect("the modulo operation will guarantee this won't go past 6")
    }

    /// Get the next date.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::{error, Zemen, Werh};
    /// let qen = Zemen::from_eth_cal(1992, Werh::Tir, 15)?.next();
    /// assert_eq!(qen, Zemen::from_eth_cal(1992, Werh::Tir, 16)?);
    /// # Ok::<(), error::Error>(())
    /// ```
    pub fn next(self) -> Self {
        Self::from_jdn(self.to_jdn() + 1).expect("incrementing by one won't panic")
    }

    /// Get the previous date.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::{error, Zemen, Werh};
    /// let qen = Zemen::from_eth_cal(1992, Werh::Tir, 15)?.previous();
    /// assert_eq!(qen, Zemen::from_eth_cal(1992, Werh::Tir, 14)?);
    /// # Ok::<(), error::Error>(())
    /// ```
    pub fn previous(self) -> Self {
        Self::from_jdn(self.to_jdn() - 1).expect("decrementing by one won't panic")
    }

    /// Get the day of the year.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::{error, Zemen, Werh};
    /// let qen = Zemen::from_eth_cal(1992, Werh::Meskerem, 15)?;
    /// assert_eq!(qen.ordinal(), 15);
    /// # Ok::<(), error::Error>(())
    /// ```
    pub fn ordinal(&self) -> u16 {
        (self.ordinal_date & 0x1ff) as _
    }

    /// Get the year, and day of the year.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::{error, Zemen, Werh};
    /// let qen = Zemen::from_eth_cal(1992, Werh::Meskerem, 15)?;
    /// assert_eq!(qen.ordinal_date(), (1992, 15));
    /// # Ok::<(), error::Error>(())
    /// ```
    pub fn ordinal_date(&self) -> (i32, u16) {
        (self.year(), self.ordinal())
    }

    /// Formats the current date given a format specifires.
    ///
    /// currently the supported format specifires are:
    /// ```txt
    ///
    /// YY       The last two digits of year (00..99)
    /// YYYY     Full Year
    /// M        Month (01..12)
    /// MM       Abbreviated month name (e.g., መስከ)
    /// MMM      Full Month Name (e.g., መስከረም)
    /// D        Day of Month (1..31)
    /// DD       Day of Week Abbreviated (e.g., ማክሰ)
    /// DDD      Day of Week (e.g., ማክሰ)
    /// JJ       Day of Year (001..366)
    /// QQ       Quarter of Year (1..4)
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use zemen::*;
    /// let qen = Zemen::from_eth_cal(2015, Werh::Tir, 10)?;
    /// assert_eq!(&qen.format("ዛሬ ቀን DD, MM D-YYYY ነው")[..], "ዛሬ ቀን ረቡዕ, ጥር 10-2015 ነው");
    /// # Ok::<(), error::Error>(())
    /// ```
    pub fn format(&self, pattern: &str) -> String {
        formatting::format(self, pattern)
    }
}

#[cfg(test)]
mod tests {
    use crate::error;
    use crate::error::Error;
    use crate::Werh;
    use crate::Zemen;

    #[test]
    // #[cfg(feature = "time")]
    fn test_get_current_date_and_convert_to_gre() {
        let zare = Zemen::today();
        println!("Zare (initiated): {}", zare);

        let converted_date = zare.to_gre();
        println!("Today (converted): {}", converted_date);

        let converted_zare = Zemen::from_date(&converted_date);
        println!("Zare (converted): {}", converted_zare);
    }

    #[test]
    fn test_month_creating_and_basic_parsing() -> Result<(), error::Error> {
        let wer_num = Werh::try_from(13)?;
        let wer_enum_pag = Werh::Puagme;

        assert_eq!(wer_enum_pag, wer_num);

        let wer_enum_mesk = Werh::Meskerem;
        let wer_str: Werh = "Meskerem".parse()?;

        assert_eq!(wer_enum_mesk, wer_str);

        Ok(())
    }

    #[test]
    fn test_zemen_date_range_error() {
        let err = error::Error::InvalidRange {
            max: 30,
            min: 1,
            given: 31,
            name: "month",
        };

        println!("{}", err)
    }

    #[test]
    #[cfg(feature = "time")]
    fn test_trait_conversion() -> Result<(), error::Error> {
        use time::Date;

        let qen = Zemen::from_eth_cal(1992, Werh::Tahasass, 22)?;
        let day: time::Date = Date::from(&qen);

        println!("qen: {}", qen);
        println!("day: {}", day);
        Ok(())
    }

    #[test]
    fn test_ordinal_date_creation() {
        let qen = Zemen::from_ordinal_date(2001, 366);
        assert!(qen.is_err());

        let qen = Zemen::from_ordinal_date(2003, 367);
        assert!(qen.is_err());

        let qen = Zemen::from_ordinal_date(2003, 366);
        assert!(qen.is_ok());

        let qen = Zemen::from_ordinal_date(2001, 365);
        assert!(qen.is_ok());
    }

    #[test]
    fn test_adding_days_to_zemen() -> Result<(), Error> {
        let qen = Zemen::from_eth_cal(2000, Werh::Meskerem, 1)?;
        let qen = qen + 30;

        assert_eq!(qen, Zemen::from_eth_cal(2000, Werh::Tikimit, 1)?);

        let qen = Zemen::from_eth_cal(2000, Werh::Puagme, 1)?;
        let qen = qen + 6;

        assert_eq!(qen, Zemen::from_eth_cal(2001, Werh::Meskerem, 2)?);

        let qen = Zemen::from_eth_cal(2003, Werh::Puagme, 1)?;
        let qen = qen + 6;

        assert_eq!(qen, Zemen::from_eth_cal(2004, Werh::Meskerem, 1)?);

        Ok(())
    }
}
