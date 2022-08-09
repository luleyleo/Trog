use adw::prelude::*;
use gdk_pixbuf::{
    gio::{Cancellable, MemoryInputStream},
    glib::Bytes,
    Pixbuf,
};

mod futures;
mod markdown;
mod model;
mod ui;

fn main() {
    gtk::init().unwrap();

    let application = adw::Application::builder()
        .application_id("de.leopoldluley.Trog")
        .build();

    let model = model::AppModel::default();
    model.channels().append(&fetch_model());

    application.connect_startup(load_resources);
    application.connect_activate(move |app| ui::setup(app, &model));

    application.run();
}

fn fetch_model() -> model::Channel {
    let rss_channel = futures::block_on(feeds::fetch_channel(feeds::DEMO_URL)).unwrap();

    let channel = model::Channel::new(
        &rss_channel.title,
        &rss_channel.link,
        &rss_channel.description,
    );

    if let Some(image) = rss_channel.image {
        let bytes = Bytes::from_owned(image);
        let stream = MemoryInputStream::from_bytes(&bytes);
        let pixbuf = &Pixbuf::from_stream(&stream, None::<&Cancellable>).unwrap();
        channel.set_image(pixbuf);
    }

    for rss_item in rss_channel.items {
        let item = model::Item::new(&rss_item.title, &rss_item.link);
        channel.items().append(&item);
    }

    channel
}

fn load_resources(_app: &adw::Application) {
    adw::init();

    let resources_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/resources.gresource"));
    let resources_data = gtk::glib::Bytes::from(resources_bytes);
    let resources = gtk::gio::Resource::from_data(&resources_data).unwrap();
    gtk::gio::resources_register(&resources);

    let display = gtk::gdk::Display::default().expect("Could not connect to a display.");
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/de/leopoldluley/trog/style.css");
    gtk::StyleContext::add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
