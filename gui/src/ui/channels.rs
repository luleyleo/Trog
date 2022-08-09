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

    let add_button = gtk::Button::builder()
        .icon_name("list-add-symbolic")
        .build();
    let menu_button = gtk::Button::builder()
        .icon_name("open-menu-symbolic")
        .build();

    let header = adw::HeaderBar::builder()
        .title_widget(&gtk::Label::new(None))
        .build();
    header.pack_start(&add_button);
    header.pack_end(&menu_button);
    leaflet
        .bind_property("folded", &header, "show-end-title-buttons")
        .flags(BindingFlags::SYNC_CREATE)
        .build();

    let list = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();

    list.bind_model(
        Some(&model.channels()),
        clone!(@strong leaflet, @strong model => move |item| {
            let channel = item.clone().downcast::<model::Channel>().unwrap();

            let row = adw::ActionRow::builder()
                .activatable(true)
                .title(&markdown::escape(channel.title()))
                .subtitle(&markdown::escape(channel.description()))
                .subtitle_lines(1)
                .build();

            row.add_prefix(&gtk::Image::from_pixbuf(Some(&channel.image())));

            row.connect_activated(clone!(@strong leaflet, @strong model, @strong channel => move |_| {
                model.set_title(&channel.title());
                model.items().clear();
                model.items().extend_from_model(&channel.items());

                if leaflet.is_folded() {
                    leaflet.navigate(adw::NavigationDirection::Forward);
                }
            }));

            row.upcast()
        }),
    );

    let scrolled_list = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .child(&list)
        .vexpand(true)
        .build();

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .width_request(250)
        .build();
    content.append(&header);
    content.append(&scrolled_list);

    content
}
