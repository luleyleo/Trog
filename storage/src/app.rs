use glib::ObjectExt;

use crate::{ChannelsModel, ItemsModel};

glib::wrapper! {
    pub struct AppModel(ObjectSubclass<imp::AppModel>);
}

impl Default for AppModel {
    fn default() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}

impl AppModel {
    pub fn channels(&self) -> ChannelsModel {
        self.property("channels")
    }

    pub fn items(&self) -> ItemsModel {
        self.property("items")
    }

    pub fn title(&self) -> String {
        self.property("title")
    }

    pub fn set_title(&self, title: &str) {
        self.set_property("title", title);
    }
}

mod imp {
    use std::cell::RefCell;

    use gio::subclass::prelude::{ObjectImpl, ObjectSubclass};
    use glib::{ParamFlags, ParamSpec, ParamSpecGType, ParamSpecString, StaticType, ToValue};
    use once_cell::sync::Lazy;

    use crate::{ChannelsModel, ItemsModel};

    #[derive(Default)]
    pub struct AppModel {
        channels: ChannelsModel,
        items: ItemsModel,
        title: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppModel {
        const NAME: &'static str = "TrogAppModel";
        type Type = super::AppModel;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for AppModel {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecGType::builder("channels")
                        .is_a_type(ChannelsModel::static_type())
                        .flags(ParamFlags::READABLE)
                        .build(),
                    ParamSpecGType::builder("items")
                        .is_a_type(ItemsModel::static_type())
                        .flags(ParamFlags::READABLE)
                        .build(),
                    ParamSpecString::builder("title")
                        .flags(ParamFlags::READWRITE)
                        .build(),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &ParamSpec,
        ) {
            match pspec.name() {
                "title" => {
                    self.title.replace(value.get().unwrap());
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "channels" => self.channels.to_value(),
                "items" => self.items.to_value(),
                "title" => self.title.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}
