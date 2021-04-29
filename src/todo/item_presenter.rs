use super::item::Item;

use chrono::Local;

pub struct ItemPresenter<'a> {
    item: &'a Item,
    id_spacing: usize,
}

impl<'a> ItemPresenter<'a> {
    pub fn new(item: &'a Item, id_spacing: usize) -> Self {
        Self {
            item,
            id_spacing,
        }
    }

    pub fn present(&self) -> String {
        format!(
            "{:id_width$} {} {} {} {}",
            self.present_id(),
            self.present_completed(),
            self.present_desc(),
            self.present_date(),
            self.present_tags(),
            id_width = self.id_spacing
        )
    }

    fn present_id(&self) -> String {
        self.item.id.to_string()
    }

    fn present_completed(&self) -> String {
        if self.item.completed {
            "[X]".to_string()
        } else {
            "[ ]".to_string()
        }
    }

    fn present_desc(&self) -> String {
        self.item.desc.clone()
    }

    fn present_date(&self) -> String {
        let _today = Local::today().naive_local();
        let _yesterday = _today.pred();
        let _tomorrow = _today.succ();

        let date_str = match self.item.date {
            a if a == _yesterday => "Yesterday".to_string(),
            a if a == _today => "Today".to_string(),
            a if a == _tomorrow => "Tomorrow".to_string(),
            _ => self.item.date.to_string(),
        };

        let mut date = String::from("@");
        date.push_str(&date_str);
        date
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
        tags_augmented.join(" ")
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
        let item_presenter = ItemPresenter::new(&item, 1);

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
        let item_presenter = ItemPresenter::new(&item, 1);

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
        let item_presenter = ItemPresenter::new(&item, 1);

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
        let item_presenter = ItemPresenter::new(&item, 1);

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
        let item_presenter = ItemPresenter::new(&item, 1);

        println!("{}", item_presenter.present());
        assert!(&item_presenter.present().contains("[X]"));
    }
}
