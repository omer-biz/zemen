# Zemen - የ ኢትዮጵያ ቀን መቁጠሪያ

## Introduction

A date conversion crate to convert between ethiopian and gregorian dates. We
have a custom `struct` to represent ethiopian date, i.e. `Zemen`, and we are
using an external crate `time`, and specifically `time::Date`, to represent
gregorian dates.

The crate uses the [Beyene-Kudlek](http://www.geez.org/Calendars/) algorithm to
convert between jdn (Julian Day number) and ethiopic calender. And the
[time](https://github.com/time-rs/time) crate to convert between jdn (Julian
Day number) and gregorian date.

## Installation

```sh
cargo add zemen
```

## Usage

```rust
use time::{Date, Month};
use zemen::{Zemen, Werh, error};

fn main() -> Result<(), error::Error> {
    // creating dates
    // Werh means month in Ge'ez
    let qen = Zemen::from_eth_cal(1992, Werh::Tahasass, 22)?;
    let date = Date::from_calendar_date(2000, Month::January, 1)?;

    // conversion
    let converted_day = Date::from(&qen);
    let converted_qen = Zemen::from(&date);

    println!("date: {}", converted_day);
    println!("qen: {}", converted_qen);

    // accessing individual element
    println!("year: {}", qen.year());
    println!("month: {}", qen.month());
    println!("month(number): {}", qen.month() as u8);
    println!("day: {}", qen.day());

    Ok(())
}
```
