// %%     a literal %
// %m     month (01..13)
// %Y     year
// %d     day of month (e.g., 01)
// %B     full month name (e.g., መስከረም)
// %b     abbreviated month name (e.g., መስከ)
// %A     full weekday name (e.g., ማክሰኞ)
// %a     abbreviated weekday name (e.g., ማክሰ)
// %j     day of year (001..366)
// %y     last two digits of year (00..99)
// %q     quarter of year (1..4)

use crate::Zemen;

pub(crate) fn format(qen: &Zemen, pattern: &str) -> String {
    let mut formated = String::new();
    let mut byte_sequence = pattern.chars();

    while let Some(ch) = byte_sequence.next() {
        if ch == '%' {
            match byte_sequence.next() {
                Some('%') => formated.push('%'),
                Some('m') => formated.push_str(&(qen.month() as u8).to_string()),
                Some('Y') => formated.push_str(&qen.year().to_string()),
                Some('y') => formated.push_str(&(qen.year() % 100).to_string()),
                Some('d') => formated.push_str(&qen.day().to_string()),
                Some('B') => formated.push_str(&qen.month().to_string()),
                Some('b') => formated.push_str(&qen.month().short_name()),
                Some('A') => formated.push_str(&qen.weekday().to_string()),
                Some('a') => formated.push_str(&qen.weekday().short_name()),
                Some('j') => formated.push_str(&qen.ordinal().to_string()),
                Some('q') => formated.push_str(&(((qen.ordinal() * 4) / 360) + 1).to_string()),
                Some(oth) => formated.push(oth),
                _ => (),
            }
        } else {
            formated.push(ch);
        }
    }

    formated
}

#[cfg(test)]
mod tests {
    use crate::{Werh, Zemen};

    use super::*;

    #[test]
    fn test_format_specifiers() {
        // with ascii
        for i in 1..=13 {
            let qen = Zemen::from_eth_cal(2001, Werh::try_from(i).unwrap(), 1).unwrap();
            let out = format(&qen, "% %j <%q>  %d-%m-%Y<<<%y>>>>->  %B  %b %A %a %%aa %z");

            assert_eq!(
                out,
                format!(
                    " {} <{}>  {}-{}-{}<<<{}>>>>->  {}  {} {} {} %aa z",
                    qen.ordinal(),
                    (qen.ordinal() * 4 / 360) + 1,
                    qen.day(),
                    qen.month() as u8,
                    qen.year(),
                    qen.year() % 100,
                    qen.month(),
                    qen.month().short_name(),
                    qen.weekday(),
                    qen.weekday().short_name()
                )
            );
        }

        // with unicode
        for i in 1..=12 {
            let qen = Zemen::from_eth_cal(2003, Werh::try_from(i).unwrap(), i + 10).unwrap();
            let out = format(&qen, "ዛሬ ቀን %A, %B %d %Y ነው");

            assert_eq!(
                out,
                format!(
                    "ዛሬ ቀን {}, {} {} {} ነው",
                    qen.weekday(),
                    qen.month(),
                    qen.day(),
                    qen.year()
                )
            );
        }
    }
}
