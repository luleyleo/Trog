use adw::prelude::*;
use gtk::glib::{clone, BindingFlags};

use crate::{
    markdown,
    model::{self, AppModel},
};

pub fn list(leaflet: &adw::Leaflet, model: &AppModel) -> impl IsA<gtk::Widget> {
    let row = adw::ActionRow::builder()
        .activatable(true)
        .title("Click me")
        .build();
    row.connect_activated(|_| {
        eprintln!("Clicked!");
    });

    let back_button = gtk::Button::builder()
        .icon_name("go-previous-symbolic")
        .build();
    let refresh_button = gtk::Button::builder()
        .icon_name("view-refresh-symbolic")
        .build();
    let title = gtk::Label::builder()
        .css_classes(vec!["title".into()])
        .build();
    model.bind_property("title", &title, "label").build();

    let header = adw::HeaderBar::builder().title_widget(&title).build();
    header.pack_start(&back_button);
    header.pack_end(&refresh_button);

    leaflet
        .bind_property("folded", &back_button, "visible")
        .flags(BindingFlags::SYNC_CREATE)
        .build();
    back_button.connect_clicked(clone!(@strong leaflet => move |_| {
        leaflet.navigate(adw::NavigationDirection::Back);
    }));

    let list = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();

    list.bind_model(Some(&model.items()), |item| {
        let item = item.clone().downcast::<model::Item>().unwrap();

        let toggle = gtk::CheckButton::builder()
            .css_classes(vec!["read-toggle".into()])
            .build();
        toggle
            .bind_property("active", &item, "read")
            .flags(BindingFlags::BIDIRECTIONAL)
            .build();

        let row = adw::ActionRow::builder()
            .activatable(true)
            .title(&markdown::escape(item.title()))
            .build();

        row.add_prefix(&toggle);

        row.connect_activated(move |row| {
            let window = row
                .root()
                .unwrap()
                .downcast::<adw::ApplicationWindow>()
                .unwrap();
            gtk::show_uri(Some(&window), &item.link(), gtk::gdk::CURRENT_TIME);
            item.set_read(true);
        });

        row.upcast()
    });

    let scrolled_list = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .child(&list)
        .vexpand(true)
        .build();

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .build();
    content.append(&header);
    content.append(&scrolled_list);

    content
}
