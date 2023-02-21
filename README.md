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

Docs found [here](https://docs.rs/zemen/latest/zemen/)

## Installation

```sh
cargo add zemen
```

## Usage

```rust
use time::{Date, Month};
use zemen::{Zemen, Werh};
use zemen::error;

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

  // get the next, and previous date. `next` and `previous` consume `self`
  let nege = qen.next();
  println!("nege: {}", nege);
  let tilant = nege.previous().previous();
  println!("tilant: {}", tilant);

  // get the next month
  let qetay_wer = tilant.month().next();
  println!("wer: {}", qetay_wer);

  // get the previous month
  let yalef_wer = tilant.month().previous();
  println!("wer: {}", yalef_wer);
  Ok(())
}
```
## Formatting

```rust
use zemen::*;
fn main() -> Result<(), error::Error> {
  let qen = Zemen::from_eth_cal(2015, Werh::Tir, 10)?;
  let formatted = qen.format("ዛሬ %a, %b %d-%Y ነው");

  // prints: ዛሬ ረቡዕ, ጥር 10-2015 ነው
  println!("{}", formatted);
  Ok(())
}
```
