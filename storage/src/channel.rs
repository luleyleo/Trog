use gdk_pixbuf::Pixbuf;
use glib::ObjectExt;

use crate::items::ItemsModel;

const DEFAULT_IMAGE: &str =
    "/de/leopoldluley/trog/icons/scalable/actions/application-rss-symbolic.svg";

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

    pub fn image(&self) -> Pixbuf {
        self.property("image")
    }

    pub fn set_image(&self, image: &Pixbuf) {
        self.set_property("image", image);
        self.notify("image");
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

    use gdk_pixbuf::Pixbuf;
    use glib::{
        subclass::{prelude::ObjectImpl, types::ObjectSubclass},
        ParamFlags, ParamSpec, ParamSpecGType, ParamSpecString, StaticType, ToValue,
    };
    use once_cell::sync::Lazy;

    use crate::{channel::DEFAULT_IMAGE, items::ItemsModel};

    pub struct Channel {
        pub title: RefCell<String>,
        pub image: RefCell<Pixbuf>,
        pub link: RefCell<String>,
        pub description: RefCell<String>,
        pub items: ItemsModel,
    }

    impl Default for Channel {
        fn default() -> Self {
            Self {
                title: Default::default(),
                image: RefCell::new(Pixbuf::from_resource(DEFAULT_IMAGE).unwrap()),
                link: Default::default(),
                description: Default::default(),
                items: Default::default(),
            }
        }
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
                    ParamSpecGType::builder("image")
                        .is_a_type(Pixbuf::static_type())
                        .build(),
                    ParamSpecString::builder("link").build(),
                    ParamSpecString::builder("description").build(),
                    ParamSpecGType::builder("items")
                        .is_a_type(ItemsModel::static_type())
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
                    self.title.replace(value.get().unwrap());
                }
                "image" => {
                    self.image.replace(value.get().unwrap());
                }
                "link" => {
                    self.link.replace(value.get().unwrap());
                }
                "description" => {
                    self.description.replace(value.get().unwrap());
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "title" => self.title.borrow().to_value(),
                "image" => self.image.borrow().to_value(),
                "link" => self.link.borrow().to_value(),
                "description" => self.description.borrow().to_value(),
                "items" => self.items.to_value(),
                _ => unimplemented!(),
            }
        }
    }
}
