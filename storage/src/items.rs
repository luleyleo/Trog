use gio::traits::ListModelExt;
use glib::subclass::types::ObjectSubclassIsExt;

use crate::Item;

glib::wrapper! {
    pub struct ItemsModel(ObjectSubclass<imp::ItemsModel>)
        @implements gio::ListModel;
}

impl Default for ItemsModel {
    fn default() -> Self {
        glib::Object::new(&[]).expect("Failed to create `ItemsModel`.")
    }
}

impl ItemsModel {
    pub fn append(&self, item: &Item) {
        let pos = {
            let mut data = self.imp().0.borrow_mut();
            data.push(item.clone());
            (data.len() - 1) as u32
        };
        self.items_changed(pos, 0, 1);
    }

    pub fn extend_from_slice(&self, items: &[Item]) {
        let len = items.len();
        let pos = {
            let mut data = self.imp().0.borrow_mut();
            let pos = data.len();
            data.extend_from_slice(items);
            pos as u32
        };
        self.items_changed(pos, 0, len as u32);
    }

    pub fn extend_from_model(&self, items: &ItemsModel) {
        let len = items.len();
        let pos = {
            let mut data = self.imp().0.borrow_mut();
            let pos = data.len();
            data.extend_from_slice(items.imp().0.borrow().as_slice());
            pos as u32
        };
        self.items_changed(pos, 0, len as u32);
    }

    pub fn remove(&self, pos: u32) {
        self.imp().0.borrow_mut().remove(pos as usize);
        self.items_changed(pos, 1, 0);
    }

    pub fn clear(&self) {
        let mut data = self.imp().0.borrow_mut();
        let len = data.len();
        data.clear();
        self.items_changed(0, len as u32, 0);
    }

    pub fn len(&self) -> usize {
        self.imp().0.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

mod imp {
    use std::cell::RefCell;

    use gio::subclass::prelude::{ListModelImpl, ObjectImpl};
    use glib::{subclass::types::ObjectSubclass, Cast, StaticType};

    use crate::Item;

    #[derive(Default)]
    pub struct ItemsModel(pub RefCell<Vec<Item>>);

    #[glib::object_subclass]
    impl ObjectSubclass for ItemsModel {
        const NAME: &'static str = "TrogItemsModel";
        type Type = super::ItemsModel;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for ItemsModel {}

    impl ListModelImpl for ItemsModel {
        fn item_type(&self, _list_model: &Self::Type) -> glib::Type {
            Item::static_type()
        }

        fn n_items(&self, _list_model: &Self::Type) -> u32 {
            self.0.borrow().len() as u32
        }

        fn item(&self, _list_model: &Self::Type, position: u32) -> Option<glib::Object> {
            self.0
                .borrow()
                .get(position as usize)
                .map(|o| o.clone().upcast::<glib::Object>())
        }
    }
}
