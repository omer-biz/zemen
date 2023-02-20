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
    let byte_sequence = pattern.as_bytes();

    let mut index = 0;
    while let Some(ch) = byte_sequence.get(index) {
        if ch == &b'%' {
            if let Some(next_ch) = byte_sequence.get(index + 1) {
                match next_ch {
                    b'%' => formated.push('%'),
                    b'm' => formated.push_str(&(qen.month() as u8).to_string()),
                    b'Y' => formated.push_str(&qen.year().to_string()),
                    b'y' => formated.push_str(&(qen.year() % 100).to_string()),
                    b'd' => formated.push_str(&qen.day().to_string()),
                    b'B' => formated.push_str(&qen.month().to_string()),
                    b'b' => formated.push_str(&qen.month().short_name()),
                    b'A' => formated.push_str(&qen.weekday().to_string()),
                    b'a' => formated.push_str(&qen.weekday().short_name()),
                    b'j' => formated.push_str(&qen.ordinal().to_string()),
                    b'q' => formated.push_str(&(((qen.ordinal() * 4) / 360) + 1).to_string()),
                    _ => (),
                }
                index += 1;
            }
        } else {
            formated.push(*ch as char);
        }
        index += 1;
    }

    formated
}

#[cfg(test)]
mod tests {
    use crate::{Werh, Zemen};

    use super::*;

    #[test]
    fn some_test() {
        for i in 1..=13 {
            let qen = Zemen::from_eth_cal(2001, Werh::try_from(i).unwrap(), 1).unwrap();
            let out = format(&qen, "% %j <%q>  %d-%m-%Y<<<%y>>>>->  %B  %b %A %a %%aa");
            println!("{}", out);
        }
        let qen = Zemen::from_eth_cal(13, Werh::try_from(3).unwrap(), 1).unwrap();
        let out = format(&qen, "% %j <%q>  %d-%m-%Y<<<%y>>>>->  %B  %b %A %a %%a");
        println!("{}", out);
    }
}
