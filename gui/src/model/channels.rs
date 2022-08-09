use gio::traits::ListModelExt;
use glib::subclass::types::ObjectSubclassIsExt;

use crate::model::Channel;

glib::wrapper! {
    pub struct ChannelsModel(ObjectSubclass<imp::ChannelsModel>)
        @implements gio::ListModel;
}

impl Default for ChannelsModel {
    fn default() -> Self {
        glib::Object::new(&[]).expect("Failed to create `ChannelsModel`.")
    }
}

impl ChannelsModel {
    pub fn append(&self, channel: &Channel) {
        let pos = {
            let mut data = self.imp().0.borrow_mut();
            data.push(channel.clone());
            (data.len() - 1) as u32
        };
        self.items_changed(pos, 0, 1);
    }

    pub fn extend_from_slice(&self, channels: &[Channel]) {
        let len = channels.len();
        let pos = {
            let mut data = self.imp().0.borrow_mut();
            let pos = data.len();
            data.extend_from_slice(channels);
            pos as u32
        };
        self.items_changed(pos, 0, len as u32);
    }

    pub fn remove(&self, pos: u32) {
        self.imp().0.borrow_mut().remove(pos as usize);
        self.items_changed(pos, 1, 0);
    }
}

mod imp {
    use std::cell::RefCell;

    use gio::subclass::prelude::{ListModelImpl, ObjectImpl};
    use glib::{subclass::types::ObjectSubclass, Cast, StaticType};

    use crate::model::Channel;

    #[derive(Default)]
    pub struct ChannelsModel(pub RefCell<Vec<Channel>>);

    #[glib::object_subclass]
    impl ObjectSubclass for ChannelsModel {
        const NAME: &'static str = "TrogChannelsModel";
        type Type = super::ChannelsModel;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for ChannelsModel {}

    impl ListModelImpl for ChannelsModel {
        fn item_type(&self, _list_model: &Self::Type) -> glib::Type {
            Channel::static_type()
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
