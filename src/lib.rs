use std::fmt;
use time;

mod conversion;
mod werh;
pub mod error;
pub use werh::Werh;

/// An Ethiopian Date.
#[derive(Debug, PartialEq, Clone)]
pub struct Zemen {
    year: i32,
    month: Werh,
    day: u8,
}

impl fmt::Display for Zemen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}/{}", self.month, self.day, self.year)
    }
}

impl From<&Zemen> for time::Date {

    /// Converts `zemen::Zemen`, which is in Ethiopian format,
    /// to it's Gregorain format.
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::error;
    /// # use time::Date;
    /// # use time::Month;
    /// # use zemen::Werh;
    /// # fn main() -> Result<(), error::Error> {
    /// let qen = Zemen::from_eth_cal(1992, Werh::Tahasass, 22)?;
    /// let day = Date::from(&qen);
    ///
    /// assert_eq!(2000, day.year());
    /// assert_eq!(Month::January, day.month());
    /// assert_eq!(1, day.day());
    /// # Ok(())
    /// # }
    /// ```
    fn from(value: &Zemen) -> Self {
        Zemen::from_eth_date(&value)
    }
}

// impl Into<time::Date> for Zemen {
//     fn into(self) -> time::Date {
//         Self::from_eth_date(&self)
//     }
// }

impl From<&time::Date> for Zemen {

    /// Converts `time::Date`, which is in Greogrian format,
    /// to it's Ethiopian format.
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::error;
    /// # use zemen::Werh;
    /// # use time::Date;
    /// # use time::Month;
    /// # fn main() -> Result<(), error::Error> {
    /// let day = Date::from_calendar_date(2000, Month::January, 1)?;
    /// let qen = Zemen::from(&day);
    ///
    /// assert_eq!(1992, qen.year());
    /// assert_eq!(Werh::Tahasass, qen.month());
    /// assert_eq!(22, qen.day());
    /// # Ok(())
    /// # }
    /// ```
    fn from(value: &time::Date) -> Self {
        Self::from_gre_date(value)
    }
}

impl Zemen {
    fn new(year: i32, month: u8, day: u8) -> Result<Self, error::Error> {
        // TODO: validate Ethiopian date
        let month = Werh::try_from(month)?;
        Ok(Zemen { year, month, day })
    }


    /// Get the year.
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::Werh;
    /// # use zemen::error;
    /// # fn main() -> Result<(), error::Error> {
    /// let qen: Zemen = Zemen::from_eth_cal(2000, Werh::Meskerem, 1)?;
    /// assert_eq!(qen.year(), 2000);
    /// Ok(())
    /// # }
    /// ```
    pub fn year(&self) -> i32 { self.year }

    /// Get the month.
    ///
    /// ```rust
    /// # use zemen::{Zemen, Werh, error};
    ///
    /// # fn main() -> Result<(), error::Error> {
    /// let qen: Zemen = Zemen::from_eth_cal(2000, Werh::Meskerem, 1)?;
    /// assert_eq!(qen.month(), Werh::Meskerem);
    /// Ok(())
    /// # }
    /// ```
    pub fn month(&self) -> Werh {
        self.month
    }

    /// Get the day of the month.
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::Werh;
    /// # use zemen::error;
    /// # fn main() -> Result<(), error::Error> {
    ///
    /// let qen: Zemen = Zemen::from_eth_cal(2000, Werh::Meskerem, 30)?;
    /// assert_eq!(qen.day(), 30);
    /// # Ok(())
    /// # }
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
        ).expect("Since `today` is valid conversion won't fail")
    }

    /// Takes a gregorian calendar, converts into Ethiopian calendar and
    /// returns a `Zemen` instance.
    ///
    /// ```rust
    /// # // use zemen::Zemen;
    /// # // use zemen::error;
    /// # // use time::{Date, Month};
    ///
    /// # // fn main() -> Result<(), error::Error> {
    /// # // let date: Date = Date::from_calendar_date(2000, Month::January, 1)?;
    /// # // let qen: Zemen = Zemen::from_gre_cal(2000, 1, 1)?;
    ///
    /// # // assert_eq!(date, qen.to_gre());
    /// # // Ok(())
    /// # // }
    /// ```
    // fn from_gre_cal(year: i32, month: u8, day: u8) -> Result<Self, error::Error> {
    //     conversion::gre_to_eth(
    //         year, month, day
    //     )
    // }

    /// Converts `&time::Date` (Gregorian date) to `zemen::Zemen` (Ethiopian date)
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::error;
    /// # use time::{Date, Month};
    ///
    /// # fn main() -> Result<(), error::Error> {
    /// let date: Date = Date::from_calendar_date(2000, Month::January, 1)?;
    /// let qen: Zemen = Zemen::from_gre_date(&date);
    /// assert_eq!(date, qen.to_gre());
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_gre_date(gc_date: &time::Date) -> Self {
        conversion::gre_to_eth(
            gc_date.year(),
            gc_date.month() as u8,
            gc_date.day(),
        ).expect("Since `gc_date` is a valid date the returned date will also be valid")
    }

    /// Converts `&zemen::Zemen` (Ethiopian date) to `time::Date` (Gregorian date)
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::error;
    /// # use time::{Date, Month};
    ///
    /// # fn main() -> Result<(), error::Error> {
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_eth_date(et_date: &Zemen) -> time::Date {
        conversion::eth_to_gre(
            et_date.year(),
            et_date.month() as u8,
            et_date.day(),
        ).expect("`et_date` is valid no need to error")
    }

    /// Create an Ethiopian date from it's number representations
    ///
    /// ```rust
    /// # use zemen::{Zemen, Werh, error};
    /// # fn main() -> Result<(), error::Error> {
    /// let qen = Zemen::from_eth_cal(1992, Werh::Tahasass, 22)?;
    ///
    /// assert_eq!(qen.year(), 1992);
    /// assert_eq!(qen.month(), Werh::Tahasass);
    /// assert_eq!(qen.day(), 22);
    /// # Ok(())
    /// # }
    ///
    /// ```
    pub fn from_eth_cal(year: i32, month: Werh, day: u8) -> Result<Self, error::Error> {
        Ok(Self::new(year, month as u8, day)?)
    }

    /// Convertes the current Ethiopian date in `time::Date`.
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::error;
    /// # use time::{Date, self};
    /// # fn main() -> Result<(), error::Error> {
    /// let qen: Zemen = Zemen::from_eth_cal(1992, zemen::Werh::Tahasass, 22)?;
    /// let date: Date = Date::from_calendar_date(2000, time::Month::January, 1)?;
    ///
    /// assert_eq!(date, qen.to_gre());
    /// # Ok(())
    /// # }
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
    /// # use zemen::Werh;
    /// # use zemen::error;
    ///
    /// # fn main() -> Result<(), error::Error> {
    /// assert_eq!(Zemen::from_jdn(2_451_545)?, Zemen::from_eth_cal(1992, Werh::Tahasass, 22)?);
    /// assert_eq!(Zemen::from_jdn(2_458_485)?, Zemen::from_eth_cal(2011, Werh::Tahasass, 23)?);
    /// assert_eq!(Zemen::from_jdn(2_458_849)?, Zemen::from_eth_cal(2012, Werh::Tahasass, 21)?);
    /// #   Ok(())
    /// # }
    /// ```
    pub fn from_jdn(jdn: i32) -> Result<Self, error::Error> {
        let (year, month, day) = conversion::jdn_to_eth(jdn);
        let month: Werh = Werh::try_from(month)?;
        Ok(Self::from_eth_cal(year, month, day)?)
    }

    /// Get the Julian day number for the Ethiopian date.
    ///
    /// ```rust
    /// # use zemen::Zemen;
    /// # use zemen::Werh;
    /// # use zemen::error;
    ///
    /// # fn main() -> Result<(), error::Error> {
    /// assert_eq!(Zemen::from_eth_cal(1992, Werh::Tahasass, 22)?.to_jdn(), 2_451_545);
    /// assert_eq!(Zemen::from_eth_cal(2011, Werh::Tahasass, 23)?.to_jdn(), 2_458_485);
    /// assert_eq!(Zemen::from_eth_cal(2012, Werh::Tahasass, 21)?.to_jdn(), 2_458_849);
    /// #   Ok(())
    /// # }
    /// ```
    pub fn to_jdn(&self) -> i32 {
        conversion::eth_to_jdn(
            self.year, self.month as i32, self.day as i32
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Zemen;
    use crate::Werh;
    use crate::error;

    #[test]
    fn test_get_current_date_and_convert_to_gre() {
        let zare = Zemen::today();
        println!("Zare (initiated): {}", zare);

        let converted_date = zare.to_gre();
        println!("Today (converted): {}", converted_date);

        let converted_zare = Zemen::from_gre_date(&converted_date);
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

        println!("wer_num: {}", wer_num);
        println!("wer_str: {}", wer_str);

        Ok(())
    }

    #[test]
    fn test_zemen_date_range_error() {
        let err = error::InvalidDateRangeError { 
            max: 30,
            min: 1,
            given: 31,
            name: "month"
        };

        println!("{}", err)
    }

    #[test]
    fn test_trait_conersion() -> Result<(), error::Error>{
        use time::Date;

        let qen = Zemen::from_eth_cal(1992, Werh::Tahasass, 22)?;
        let day: time::Date = Date::from(&qen);

        println!("qen: {}", qen);
        println!("day: {}", day);
        Ok(())
    }
}
