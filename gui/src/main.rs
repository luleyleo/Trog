use adw::prelude::*;
use gdk_pixbuf::{
    gio::{Cancellable, MemoryInputStream},
    glib::Bytes,
    Pixbuf,
};
use gtk::glib::{clone, BindingFlags};
use storage::AppModel;

mod futures;

fn main() {
    gtk::init().unwrap();

    let application = adw::Application::builder()
        .application_id("de.leopoldluley.Trog")
        .build();

    application.connect_startup(load_resources);
    application.connect_activate(setup);

    application.run();
}

fn escape_markdown(mrkd: String) -> String {
    mrkd.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn fetch_model() -> storage::Channel {
    let rss_channel = futures::block_on(feeds::fetch_channel(feeds::DEMO_URL)).unwrap();

    let channel = storage::Channel::new(
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

fn setup(app: &adw::Application) {
    let model = AppModel::default();
    model.channels().append(&fetch_model());

    let leaflet = adw::Leaflet::new();
    leaflet.append(&list_channels(&leaflet, &model));
    leaflet
        .append(&gtk::Separator::new(gtk::Orientation::Vertical))
        .set_navigatable(false);
    leaflet.append(&list_items(&leaflet, &model));

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(800)
        .content(&leaflet)
        .build();
    window.show();
}

fn list_channels(leaflet: &adw::Leaflet, model: &AppModel) -> impl IsA<gtk::Widget> {
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
            let channel = item.clone().downcast::<storage::Channel>().unwrap();

            let row = adw::ActionRow::builder()
                .activatable(true)
                .title(&escape_markdown(channel.title()))
                .subtitle(&escape_markdown(channel.description()))
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

fn list_items(leaflet: &adw::Leaflet, model: &AppModel) -> impl IsA<gtk::Widget> {
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
        let item = item.clone().downcast::<storage::Item>().unwrap();

        let toggle = gtk::CheckButton::builder()
            .css_classes(vec!["read-toggle".into()])
            .build();
        toggle
            .bind_property("active", &item, "read")
            .flags(BindingFlags::BIDIRECTIONAL)
            .build();

        let row = adw::ActionRow::builder()
            .activatable(true)
            .title(&escape_markdown(item.title()))
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
