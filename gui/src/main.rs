use adw::prelude::*;

use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use gtk::glib::BindingFlags;
use gtk::{Box, Button, CheckButton, ListBox, Orientation, SelectionMode};

mod futures;

fn main() {
    gtk::init().unwrap();

    let application = Application::builder()
        .application_id("de.leopoldluley.Trog")
        .build();

    application.connect_startup(load_resources);
    application.connect_activate(setup);

    application.run();
}

fn fetch_model() -> storage::Channel {
    let rss_channel = futures::block_on(feeds::fetch_channel(feeds::DEMO_URL)).unwrap();

    let channel = storage::Channel::new(
        &rss_channel.title,
        &rss_channel.link,
        &rss_channel.description,
    );

    for rss_item in rss_channel.items {
        let item = storage::Item::new(&rss_item.title, &rss_item.link);
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

fn setup(app: &Application) {
    let row = ActionRow::builder()
        .activatable(true)
        .title("Click me")
        .build();
    row.connect_activated(|_| {
        eprintln!("Clicked!");
    });

    let add_button = Button::builder().icon_name("list-add-symbolic").build();
    let menu_button = Button::builder().icon_name("open-menu-symbolic").build();
    let refresh_button = Button::builder().icon_name("view-refresh-symbolic").build();

    let header = HeaderBar::new();
    header.pack_start(&add_button);
    header.pack_end(&menu_button);
    header.pack_end(&refresh_button);

    let list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .build();

    let model = fetch_model();
    list.bind_model(Some(&model.items()), |item| {
        let item = item.clone().downcast::<storage::Item>().unwrap();

        let title = gtk::Label::builder()
            .label(&item.title())
            .use_markup(false)
            .css_classes(vec!["title".to_string()])
            .wrap(true)
            .build();

        let toggle = CheckButton::builder()
            .css_classes(vec!["read-toggle".into()])
            .build();
        toggle
            .bind_property("active", &item, "read")
            .flags(BindingFlags::BIDIRECTIONAL)
            .build();

        let content = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .halign(gtk::Align::Start)
            .margin_top(8)
            .margin_end(12)
            .margin_bottom(8)
            .margin_start(12)
            .build();

        content.append(&toggle);
        content.append(&title);

        let row = ActionRow::builder()
            .activatable(true)
            .child(&content)
            .build();

        row.connect_activated(move |row| {
            let window = row.root().unwrap().downcast::<ApplicationWindow>().unwrap();
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

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&header);
    content.append(&scrolled_list);

    let window = ApplicationWindow::builder()
        .application(app)
        .title(&model.title())
        .default_width(350)
        .default_height(600)
        .content(&content)
        .build();
    window.show();
}
