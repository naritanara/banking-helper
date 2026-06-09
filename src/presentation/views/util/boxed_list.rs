use relm4::prelude::*;
use gtk::prelude::*;

#[relm4::widget_template(pub)]
impl WidgetTemplate for BoxedList {
    view! {
        gtk::ListBox {
            set_selection_mode: gtk::SelectionMode::None,
            set_css_classes: &["boxed-list"],
        }
    }
}
