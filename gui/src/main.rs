use adw::prelude::*;

use adw::{ActionRow, Application, ApplicationWindow, HeaderBar};
use gtk::{Box, Button, ListBox, Orientation, SelectionMode};

mod futures;

fn main() {
    let application = Application::builder()
        .application_id("de.leopoldluley.Trog")
        .build();

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
        .margin_top(16)
        .margin_end(16)
        .margin_bottom(16)
        .margin_start(16)
        .selection_mode(SelectionMode::None)
        .css_classes(vec![String::from("boxed-list")])
        .build();

    let model = fetch_model();
    list.bind_model(Some(&model.items()), |item| {
        let item = item.clone().downcast::<storage::Item>().unwrap();

        let title = gtk::Label::builder()
            .label(&item.title())
            .use_markup(false)
            .css_classes(vec!["title".to_string()])
            .halign(gtk::Align::Start)
            .wrap(true)
            .margin_top(8)
            .margin_end(12)
            .margin_bottom(8)
            .margin_start(12)
            .build();

        let row = ActionRow::builder().activatable(true).child(&title).build();
        row.connect_activated(move |row| {
            let window = row.root().unwrap().downcast::<ApplicationWindow>().unwrap();
            gtk::show_uri(Some(&window), &item.link(), gtk::gdk::CURRENT_TIME);
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
