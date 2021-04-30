use super::todo::Todo;
use super::item_presenter::ItemPresenter;

pub struct TodoPresenter<'a> {
    todo: &'a Todo,
}

impl<'a> TodoPresenter<'a> {
    pub fn new(todo: &'a Todo) -> Self {
        Self {
            todo,
        }
    }

    pub fn present(&self) -> () {
        let separator_spacing = 4 as usize;
        let id_spacing = self.todo.last_id.to_string().len();

        let mut items = self.todo.items.clone();
        items.sort_by(|a, b| a.cmp(&b));

        for item in &items {
            let presented_item = ItemPresenter::new(&item, separator_spacing, id_spacing).present();

            println!("{}", presented_item);
        }
    }
}
