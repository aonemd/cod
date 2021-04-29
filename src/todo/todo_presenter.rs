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
        let id_spacing = self.todo.last_id.to_string().len();

        for item in &self.todo.items {
            let presented_item = ItemPresenter::new(&item, id_spacing).present();

            println!("{}", presented_item);
        }
    }
}
