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
}

mod imp {
    use std::cell::RefCell;

    use glib::{
        subclass::{prelude::ObjectImpl, types::ObjectSubclass},
        ParamSpec, ParamSpecString, ToValue,
    };
    use once_cell::sync::Lazy;

    #[derive(Default)]
    pub struct Item {
        pub title: RefCell<String>,
        pub link: RefCell<String>,
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
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "title" => self.title.borrow().to_value(),
                "link" => self.link.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}
