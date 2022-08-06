use glib::ObjectExt;

glib::wrapper! {
    pub struct Item(ObjectSubclass<imp::Item>);
}

impl Item {
    pub fn new(title: &str, link: &str) -> Self {
        glib::Object::new(&[("title", &title), ("link", &link)]).expect("Failed to create `Item`.")
    }

    pub fn title(&self) -> String {
        self.property("title")
    }

    pub fn link(&self) -> String {
        self.property("link")
    }

    pub fn read(&self) -> bool {
        self.property("read")
    }

    pub fn set_read(&self, read: bool) {
        self.set_property("read", read);
        self.notify("read");
    }
}

mod imp {
    use std::cell::{Cell, RefCell};

    use glib::{
        subclass::{prelude::ObjectImpl, types::ObjectSubclass},
        ParamSpec, ParamSpecBoolean, ParamSpecString, ToValue,
    };
    use once_cell::sync::Lazy;

    #[derive(Default)]
    pub struct Item {
        pub title: RefCell<String>,
        pub link: RefCell<String>,
        pub read: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Item {
        const NAME: &'static str = "TrogItem";
        type Type = super::Item;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for Item {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::builder("title").build(),
                    ParamSpecString::builder("link").build(),
                    ParamSpecBoolean::builder("read").build(),
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
                "read" => {
                    let read = value.get().unwrap();
                    self.read.replace(read);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "title" => self.title.borrow().to_value(),
                "link" => self.link.borrow().to_value(),
                "read" => self.read.get().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}
