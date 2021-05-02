use chrono::{NaiveDate};
use date_time_parser::DateParser;
use regex::Regex;

const DATE_PATTERN: &str = r"@[a-zA-Z0-9,-/]+";
const TAG_PATTERN: &str = r"\+[a-zA-Z0-9_]+";

pub struct Parser {
    pub desc: Option<String>,
    pub date: Option<NaiveDate>,
    pub tags: Option<Vec<String>>,
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

    fn parse_desc(input: &str) -> Option<String> {
        let date_re = Regex::new(DATE_PATTERN).unwrap();
        let tag_re = Regex::new(TAG_PATTERN).unwrap();
        let desc = date_re.replace_all(input, "");
        let desc = tag_re.replace_all(&desc, "");

        let desc = desc.trim().to_string();
        if desc.is_empty() {
            None
        } else {
            Some(desc)
        }
    }

    fn parse_date(input: &str) -> Option<NaiveDate> {
        let re = Regex::new(DATE_PATTERN).unwrap();
        if let Some(date) = re.captures(input) {
            DateParser::parse(&date[0][1..]).or(
                match NaiveDate::parse_from_str(&date[0][1..], "%Y-%m-%d") {
                    Ok(d) => Some(d),
                    _ => None,
                }
            )
        } else {
            None
        }
    }

    fn parse_tags(input: &str) -> Option<Vec<String>> {
        let mut tags: Vec<String> = vec![];
        let re = Regex::new(TAG_PATTERN).unwrap();
        for caps in re.captures_iter(input) {
            tags.push((&caps[0])[1..].to_string());
        }

        if tags.is_empty() {
            None
        } else {
            Some(tags)
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    use chrono::{Local};

    #[test]
    fn test_parse_desc() -> () {
        let parser = Parser::new("Hello world @2021-04-13 +work");
        let expected_desc = Some(String::from("Hello world"));

        assert_eq!(parser.desc, expected_desc);
    }

    #[test]
    fn test_parse_desc_in_the_end() -> () {
        let parser = Parser::new("@2021-04-13 +work Hello world");
        let expected_desc = Some(String::from("Hello world"));

        assert_eq!(parser.desc, expected_desc);
    }

    #[test]
    fn test_parse_desc_empty_returns_none() -> () {
        let parser = Parser::new("@2021-04-13 +work");
        let expected_desc = None;

        assert_eq!(parser.desc, expected_desc);
    }

    #[test]
    fn test_parse_date_formal() -> () {
        let parser = Parser::new("Hello world @2021-04-13 +personal");
        let test_date = Some(NaiveDate::from_ymd(2021, 04, 13));

        assert_eq!(parser.date, test_date);
    }

    #[test]
    fn test_parse_date_natural() -> () {
        let parser = Parser::new("Hello world @today");
        let today_date = Some(Local::today().naive_local());

        assert_eq!(parser.date, today_date);
    }

    #[test]
    fn test_parse_date_not_found_returns_none() -> () {
        let parser = Parser::new("Hello world");
        let today_date = None;

        assert_eq!(parser.date, today_date);
    }

    #[test]
    fn test_parse_tags() -> () {
        let parser = Parser::new("Hello world +inbox +work @tomorrow");
        let expected_tags = Some(vec!["inbox".to_string(), "work".to_string()]);

        assert_eq!(parser.tags, expected_tags);
    }

    #[test]
    fn test_parse_tags_empty_returns_none() -> () {
        let parser = Parser::new("Hello world @tomorrow");
        let expected_tags = None;

        assert_eq!(parser.tags, expected_tags);
    }
}
