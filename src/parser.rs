use chrono::{Local, NaiveDate};
use date_time_parser::DateParser;
use regex::Regex;

const DATE_PATTERN: &str = r"@[A-Z0-9,-/]+";
const TAG_PATTERN: &str = r"\+[a-zA-Z0-9_]+";

struct Parser {
    desc: String,
    date: NaiveDate,
    tags: Vec<String>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let desc = Self::parse_desc(input);
        let date = Self::parse_date(input);
        let tags = Self::parse_tags(input);

        Self {
            desc,
            date,
            tags,
        }
    }

    fn parse_desc(input: &str) -> String {
        let date_re = Regex::new(DATE_PATTERN).unwrap();
        let tag_re = Regex::new(TAG_PATTERN).unwrap();
        let desc = date_re.replace_all(input, "");
        let desc = tag_re.replace_all(&desc, "");

        desc.trim().to_string()
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

    fn parse_tags(input: &str) -> Vec<String> {
        let mut tags: Vec<String> = vec![];
        let re = Regex::new(TAG_PATTERN).unwrap();
        for caps in re.captures_iter(input) {
            tags.push((&caps[0])[1..].to_string());
        }

        tags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_desc() -> () {
        let parser = Parser::new("Hello world @2021-04-13 +work");
        let test_date = String::from("Hello world");

        assert_eq!(parser.desc, test_date);
    }

    #[test]
    fn test_parse_desc_in_the_end() -> () {
        let parser = Parser::new("@2021-04-13 +work Hello world");
        let test_date = String::from("Hello world");

        assert_eq!(parser.desc, test_date);
    }

    #[test]
    fn test_parse_date_formal() -> () {
        let parser = Parser::new("Hello world @2021-04-13 +personal");
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

    #[test]
    fn test_parse_tags() -> () {
        let parser = Parser::new("Hello world +inbox +work @tomorrow");
        let expected_tags = vec!["inbox".to_string(), "work".to_string()];

        assert_eq!(parser.tags, expected_tags);
    }
}
