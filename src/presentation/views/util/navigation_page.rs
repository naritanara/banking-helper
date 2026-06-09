use relm4::prelude::*;

#[relm4::widget_template(pub)]
impl WidgetTemplate for NavigationPage {
    view! {
        adw::NavigationPage {
            #[name = "contents"]
            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar,
            }
        }
    }
}
