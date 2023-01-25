use time;
use crate::Zemen;
use crate::error;
/*
* # For Ethiopian
* - http://www.geez.org/Calendars/
*/

const JDN_EPOCH_OFFSET_ETH: i32 = 1_723_856;

fn modl(i: i32, j: i32) -> i32 {
    i - (j * (i / j))
}

/// Returns the Julian day number (`jdn`) given `year`, `month`, and `day`
/// in ethiopic date format.
///
/// Doesn't not check the validity of the provided date.
pub fn eth_to_jdn(year: i32, month: i32, day: i32) -> i32 {
    (JDN_EPOCH_OFFSET_ETH + 365)
    + 365 * (year - 1)
    + (year / 4)
    + 30 * month
    + day - 31
}

/// Returns the ethiopic date, given jdn, as (year, month, day)
///
/// Doesn't check for the validity of the provided Julian day number.
pub fn jdn_to_eth(jdn: i32) -> (i32, u8, u8) {
    let r = modl(jdn - JDN_EPOCH_OFFSET_ETH, 1461);
    let n = modl(r, 365) + 365 * (r / 1460);

    let year = 4 * ((jdn - JDN_EPOCH_OFFSET_ETH) / 1461)
             + (r / 365)
             - (r / 1460);
    let month = (n / 30) + 1;
    let day = modl(n, 30) + 1;

    (year, month as u8, day as u8)
}

/// Tries to create a Gregorian date from Ethiopian date.
///
pub fn eth_to_gre(year: i32, month: u8, day: u8) -> Result<time::Date, error::Error> {
    let jdn = eth_to_jdn(year, month as i32, day as i32);
    let date = time::Date::from_julian_day(jdn)?;

    Ok(date)
}

/// Tries to create an Ethiopian date from Gregorian date.
///
pub fn gre_to_eth(year: i32, month: u8, day: u8) -> Result<Zemen, error::Error> {
    let month = time::Month::try_from(month as u8)?;
    let date = time::Date::from_calendar_date(year, month, day as u8)?;
    let (year, month, day) = jdn_to_eth(date.to_julian_day());

    Ok(Zemen::new(year, month , day)?)
}

#[cfg(test)]
mod basic_conversion {
    use super::*;

    #[test]
    fn test_gre_to_eth() -> Result<(), error::Error> {
        let zemen = Zemen::new(1992, 4, 22)?;
        assert_eq!(zemen, gre_to_eth(2000, 1, 1)?);

        let zemen = Zemen::new(2015, 5, 11)?;
        assert_eq!(zemen, gre_to_eth(2023, 1, 19)?);

        let zemen = Zemen::new(1915, 9, 7)?;
        assert_eq!(zemen, gre_to_eth(1923, 5, 15)?);

        // should fail
        // TODO: validate ethiopian dates
        // let zemen = Zemen::new(1915, 13, 7);
        // assert_eq!(zemen, gre_to_eth(1923, 5, 15)?);

        Ok(())
    }

    #[test]
    fn test_eth_to_gre() -> Result<(), error::Error> {
        let (year, month, day) = (2000, 1, 1);
        let month = time::Month::try_from(month)?;
        let gre_date = time::Date::from_calendar_date(year, month, day)?;
        assert_eq!(gre_date, eth_to_gre(1992, 4, 22)?);

        let (year, month, day) = (2023, 1, 19);
        let month = time::Month::try_from(month)?;
        let gre_date = time::Date::from_calendar_date(year, month, day)?;
        assert_eq!(gre_date, eth_to_gre(2015, 5, 11)?);

        let (year, month, day) = (1923, 5, 15);
        let month = time::Month::try_from(month)?;
        let gre_date = time::Date::from_calendar_date(year, month, day)?;
        assert_eq!(gre_date, eth_to_gre(1915, 9, 7)?);

        Ok(())
    }
}
