use adw::prelude::*;

use crate::model::AppModel;

mod channels;
mod items;

pub fn setup(app: &adw::Application, model: &AppModel) {
    let leaflet = adw::Leaflet::new();
    leaflet.append(&channels::list(&leaflet, model));
    leaflet
        .append(&gtk::Separator::new(gtk::Orientation::Vertical))
        .set_navigatable(false);
    leaflet.append(&items::list(&leaflet, model));

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(800)
        .content(&leaflet)
        .build();
    window.show();
}
