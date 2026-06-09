use relm4::prelude::*;
use gtk::prelude::*;

#[relm4::widget_template(pub)]
impl WidgetTemplate for ClampBox {
    view! {
        adw::Clamp {
            #[name = "contents"]
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_vertical: 24,
                set_margin_horizontal: 12,
                set_spacing: 24,
            }
        }
    }
}