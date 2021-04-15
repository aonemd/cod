use chrono::{Local, NaiveDate};

#[derive(Debug)]
pub struct Item {
    pub id: u32,
    pub desc: String,
    pub date: NaiveDate,
}

impl Item {
    pub fn new(id: u32, desc: String, date: &str) -> Self {
        let date =
            NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap_or(Local::today().naive_local());

        Self { id, desc, date }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_item_with_valid_date_parses_str_date() {
        let test_date_as_str = "2021-04-14";
        let test_date = NaiveDate::parse_from_str(&test_date_as_str, "%Y-%m-%d").unwrap();

        let item = Item::new(1, String::from("Hello"), test_date_as_str);

        assert_eq!(item.date, test_date);
    }

    #[test]
    fn test_new_item_with_invalid_date_sets_date_to_today() {
        let test_date_as_str = "";
        let test_date = Local::today().naive_local();

        let item = Item::new(1, String::from("Hello"), test_date_as_str);

        assert_eq!(item.date, test_date);
    }
}
