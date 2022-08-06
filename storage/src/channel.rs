use glib::ObjectExt;

use crate::items::ItemsModel;

glib::wrapper! {
    pub struct Channel(ObjectSubclass<imp::Channel>);
}

impl Channel {
    pub fn new(title: &str, link: &str, description: &str) -> Self {
        glib::Object::new(&[
            ("title", &title),
            ("link", &link),
            ("description", &description),
        ])
        .expect("Failed to create `Channel`.")
    }

    pub fn title(&self) -> String {
        self.property("title")
    }

    pub fn link(&self) -> String {
        self.property("link")
    }

    pub fn description(&self) -> String {
        self.property("description")
    }

    pub fn items(&self) -> ItemsModel {
        self.property("items")
    }
}

mod imp {
    use std::cell::RefCell;

    use glib::{
        subclass::{prelude::ObjectImpl, types::ObjectSubclass},
        ParamFlags, ParamSpec, ParamSpecGType, ParamSpecString, StaticType, ToValue,
    };
    use once_cell::sync::Lazy;

    use crate::{items::ItemsModel, Item};

    #[derive(Default)]
    pub struct Channel {
        pub title: RefCell<String>,
        pub link: RefCell<String>,
        pub description: RefCell<String>,
        pub items: ItemsModel,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Channel {
        const NAME: &'static str = "TrogChannel";
        type Type = super::Channel;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for Channel {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::builder("title").build(),
                    ParamSpecString::builder("link").build(),
                    ParamSpecString::builder("description").build(),
                    ParamSpecGType::builder("items")
                        .is_a_type(Item::static_type())
                        .flags(ParamFlags::READABLE)
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
                    let title = value.get().unwrap();
                    self.title.replace(title);
                }
                "link" => {
                    let link = value.get().unwrap();
                    self.link.replace(link);
                }
                "description" => {
                    let description = value.get().unwrap();
                    self.description.replace(description);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "title" => self.title.borrow().to_value(),
                "link" => self.link.borrow().to_value(),
                "description" => self.description.borrow().to_value(),
                "items" => self.items.to_value(),
                _ => unimplemented!(),
            }
        }
    }
}
