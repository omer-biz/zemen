// YY       The last two digits of year (00..99)
// YYYY     Full Year
// M        Month (01..12)
// MM       Abbreviated month name (e.g., መስከ)
// MMM      Full Month Name (e.g., መስከረም)
// D        Day of Month (1..31)
// DD       Day of Week Abbreviated (e.g., ማክሰ)
// DDD      Abbreviated Weekday Name (e.g., ማክሰ)
// JJ       Day of Year (001..366)
// QQ       Quarter of Year (1..4)

use crate::Zemen;

pub(crate) fn format(qen: &Zemen, pattern: &str) -> String {
    let formated = pattern
        .replace("YYYY", &qen.year().to_string())
        .replace("YY", &format!("{:02}", (qen.year() % 100)))
        .replace("MMM", &qen.month().to_string())
        .replace("MM", &qen.month().short_name())
        .replace("M", &format!("{:02}", (qen.month() as u8)))
        .replace("DDD", &qen.weekday().to_string())
        .replace("DD", &qen.weekday().short_name())
        .replace("D", &format!("{:02}", qen.day()))
        .replace("JJ", &format!("{:03}", qen.ordinal()))
        .replace("QQ", &format!("{:02}", (qen.ordinal() / 4 / 360) + 1));

    formated
}

#[cfg(test)]
mod tests {
    use crate::{Werh, Zemen};

    use super::*;

    #[test]
    fn test_format_specifiers_with_ascii() {
        for i in 1..=13 {
            let qen = Zemen::from_eth_cal(2001, Werh::try_from(i).unwrap(), 1).unwrap();
            let out = format(&qen, "YY YYYY M D DD DDD MM MMM QQ JJ");

            assert_eq!(
                out,
                format!(
                    "{} {} {} {} {} {} {} {} {} {}",
                    format!("{:02}", (qen.year() % 100)),
                    qen.year(),
                    format!("{:02}", (qen.month() as u8)),
                    format!("{:02}", qen.day()),
                    qen.weekday().short_name(),
                    qen.weekday(),
                    qen.month().short_name(),
                    qen.month(),
                    format!("{:02}", (qen.ordinal() / 4 / 360) + 1),
                    format!("{:03}", qen.ordinal()),
                )
            );
        }
    }

    #[test]
    fn test_format_specifiers_with_unicode() {
        // with unicode
        for i in 1..=12 {
            let qen = Zemen::from_eth_cal(2003, Werh::try_from(i).unwrap(), i + 10).unwrap();
            let out = format(&qen, "ዛሬ ቀን DDD, MMM D YYYY ነው");

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
