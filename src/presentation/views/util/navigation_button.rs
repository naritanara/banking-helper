use relm4::prelude::*;
use gtk::prelude::*;

#[relm4::widget_template(pub)]
impl WidgetTemplate for NavigationButton {
    type Init = &'static str;
    
    view! {
        #[name = "button"]
        gtk::Button {
            set_label: init,
            set_css_classes: &["pill", "suggested-action"],
            set_action_name: Some("navigation.push"),
        }
    }
}

impl NavigationButton {
    pub fn set_navigation_target(&self, target: Option<&'static str>) {
        self.button.set_action_target(target);
        self.button.set_sensitive(target.is_some());
    }
}