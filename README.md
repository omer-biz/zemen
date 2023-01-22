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

Since the crate is still under heavy development, and me being a beginner rust
programmer, it won't be published on `crates.io` yet. For now you can clone the
repo and link to it in your `Cargo.toml` like so.

```toml
...snip...

[dependencies]
zemen = { path = "path/to/zemen/repo" }

...snip...
```

## Usage

### Basic Usage

You can find the documantation at [here](https://omer-biz.github.io/zemen_doc/doc/zemen/index.html).

```rust
use zemen;

fn main() {
  // Returns the current Ethiopian date.
  let zare: zemen::Zemen = zemen::Zemen::today();

  // print: `ጥር 14/2015`
  println!("Zare: {}", zare);

  // To access the individal elements
  let year: i32 = zare.year(); // 2015
  // Returns the `zemen::Month::Tir` enum which could be casted as `u8`
  // to get the month representain in number
  // let month_num: u8 = zemen::Month::Tir as u8;
  let month: zemen::Month = zare.month(); // `Month::Tir`
  let day: u8 = zare.day(); // 14

  // printing the month will print the equvalent month in amharic
  println!("{}", month); // prints: `ጥር`
}
```

### Conversion

```rust
use time;
use zemen;

fn main() {
  let qen = zemen::Zemen::from_eth_cal(1992, 4, 22).unwrap()
  let day = time::Date::from_calendar_date(2000, time::Month::January, 1).unwrap();

  // to convert Ethiopian to Greogroian
  let converted_day: time::Date = qen.to_gre();

  // To convert Gregorian to Ethiopian
  let converted_qen: zemen::Zemen = zemen::Zemen::from_gre_date(&day);
}
```
