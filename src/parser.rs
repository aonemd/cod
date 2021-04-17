use chrono::{Local, NaiveDate};
use date_time_parser::DateParser;
use regex::Regex;

const DATE_PATTERN: &str = r"@[A-Z0-9,-/]+";

struct Parser {
    desc: String,
    date: NaiveDate,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let date = Self::parse_date(input);

        Self {
            desc: String::from(""),
            date,
        }
    }

    fn parse_date(input: &str) -> NaiveDate {
        let re = Regex::new(DATE_PATTERN).unwrap();
        if let Some(date) = re.captures(input) {
            DateParser::parse(&date[0]).unwrap_or(
                NaiveDate::parse_from_str(&date[0][1..], "%Y-%m-%d")
                    .unwrap_or(Local::today().naive_local()),
            )
        } else {
            Local::today().naive_local()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_formal() -> () {
        let parser = Parser::new("Hello world @2021-04-13");
        let test_date = NaiveDate::from_ymd(2021, 04, 13);

        assert_eq!(parser.date, test_date);
    }

    #[test]
    fn test_parse_date_natural() -> () {
        let parser = Parser::new("Hello world @today");
        let today_date = Local::today().naive_local();

        assert_eq!(parser.date, today_date);
    }

    #[test]
    fn test_parse_date_not_found_returns_today() -> () {
        let parser = Parser::new("Hello world");
        let today_date = Local::today().naive_local();

        assert_eq!(parser.date, today_date);
    }
}
