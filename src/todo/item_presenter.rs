use super::item::Item;

use chrono::Local;
use colored::*;

pub struct ItemPresenter<'a> {
    item: &'a Item,
    separator_spacing: usize,
    id_spacing: usize,
}

impl<'a> ItemPresenter<'a> {
    pub fn new(item: &'a Item, separator_spacing: usize, id_spacing: usize) -> Self {
        Self {
            item,
            separator_spacing,
            id_spacing,
        }
    }

    pub fn present(&self) -> String {
        let (completed, completed_len) = self.present_completed();
        let (date, date_len) = self.present_date();

        format!(
            "{:id_width$} {:completed_width$} {:date_width$} {} {}",
            self.present_id(),
            completed,
            date,
            self.present_desc(),
            self.present_tags(),
            id_width = self.id_spacing + self.separator_spacing,
            completed_width = completed_len + self.separator_spacing,
            date_width = date_len + self.separator_spacing,
        )
    }

    fn present_id(&self) -> String {
        self.item.id.to_string()
    }

    fn present_completed(&self) -> (String, usize) {
        if self.item.completed {
            ("[X]".to_string().green().to_string(), 12)
        } else {
            ("[ ]".to_string(), 3)
        }
    }

    fn present_date(&self) -> (String, usize) {
        let _today = Local::today().naive_local();
        let _yesterday = _today.pred();
        let _tomorrow = _today.succ();

        let (date, uncolored_len) = match self.item.date {
            a if a == _yesterday => {
                let mut date_str = String::from("@");
                date_str.push_str("Yesterday");
                (date_str.magenta().to_string(), date_str.len())
            }
            a if a == _today => {
                let mut date_str = String::from("@");
                date_str.push_str("Today");
                (date_str.bold().cyan().to_string(), date_str.len())
            }
            a if a == _tomorrow => {
                let mut date_str = String::from("@");
                date_str.push_str("Tomorrow");
                (date_str.cyan().to_string(), date_str.len())
            }
            _ => {
                let mut date_str = String::from("@");
                date_str.push_str(&self.item.date.to_string());
                (date_str.cyan().to_string(), date_str.len())
            }
        };

        let longest_date_str_len = 11;
        let date_len = date.len() + (longest_date_str_len - uncolored_len);

        (date, date_len)
    }

    fn present_desc(&self) -> String {
        self.item.desc.clone()
    }

    fn present_tags(&self) -> String {
        let tags = &self.item.tags;
        let tags_augmented: Vec<String> = tags
            .into_iter()
            .map(|t| {
                let mut t = t.clone();
                t.insert_str(0, "+");
                t
            })
            .collect();
        tags_augmented.join(" ").blue().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::Local;

    #[test]
    fn it_presents_item_id() {
        let item = Item::new(
            1,
            "Hello".to_string(),
            Local::today().naive_local(),
            vec![String::from("tag1")],
            false,
        );
        let item_presenter = ItemPresenter::new(&item, 2, 1);

        assert!(&item_presenter.present().contains(&1.to_string()));
    }

    #[test]
    fn it_presents_item_desc() {
        let item = Item::new(
            1,
            "Hello".to_string(),
            Local::today().naive_local(),
            vec![String::from("tag1")],
            false,
        );
        let item_presenter = ItemPresenter::new(&item, 2, 1);

        assert!(&item_presenter.present().contains("Hello"));
    }

    #[test]
    fn it_presents_item_date() {
        let item = Item::new(
            1,
            "Hello".to_string(),
            Local::today().naive_local(),
            vec![String::from("tag1")],
            false,
        );
        let item_presenter = ItemPresenter::new(&item, 2, 1);

        println!("{}", item_presenter.present());
        assert!(&item_presenter.present().contains("@Today"));
    }

    #[test]
    fn it_presents_item_tags() {
        let item = Item::new(
            1,
            "Hello".to_string(),
            Local::today().naive_local(),
            vec![String::from("tag1"), String::from("tag2")],
            false,
        );
        let item_presenter = ItemPresenter::new(&item, 2, 1);

        println!("{}", item_presenter.present());
        assert!(&item_presenter.present().contains("+tag1 +tag2"));
    }

    #[test]
    fn it_presents_item_completed() {
        let item = Item::new(
            1,
            "Hello".to_string(),
            Local::today().naive_local(),
            vec![String::from("tag1"), String::from("tag2")],
            true,
        );
        let item_presenter = ItemPresenter::new(&item, 2, 1);

        println!("{}", item_presenter.present());
        assert!(&item_presenter.present().contains("[X]"));
    }
}
