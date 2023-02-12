pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0
}

pub fn is_valid_date(year: i32, month: u8, day: u8) -> bool {
    if month > 13 || day > 30 || year < 0 {
        return false;
    } else if is_leap_year(year) {
        if month == 13 && day > 6 {
            return false;
        }
    } else if month == 13 && day > 5 {
        return false;
    }
    true
}

pub fn days_in_year(year: i32) -> u16 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

#[cfg(test)]
mod tests {
    use super::{days_in_year, is_leap_year, is_valid_date};

    #[test]
    fn validator_leap_year() {
        for year_offset in (0..=40).step_by(4) {
            assert!(is_leap_year(2000 + year_offset));
        }
    }

    #[test]
    fn validator_days_in_year() {
        assert_eq!(days_in_year(2000), 366);
        assert_ne!(days_in_year(2000), 365);

        assert_eq!(days_in_year(2001), 365);
        assert_ne!(days_in_year(2001), 366);
    }

    #[test]
    fn validator_valid_date() {
        let (year, month, day) = (2000, 13, 5);
        assert!(is_valid_date(year, month, day));

        let (year, month, day) = (2001, 13, 5);
        assert!(is_valid_date(year, month, day));

        let (year, month, day) = (2000, 13, 6);
        assert!(is_valid_date(year, month, day));

        let (year, month, day) = (2000, 13, 7);
        assert!(!is_valid_date(year, month, day));

        let (year, month, day) = (2001, 13, 6);
        assert!(!is_valid_date(year, month, day));
    }
}
