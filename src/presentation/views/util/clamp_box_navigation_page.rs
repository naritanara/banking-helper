use relm4::prelude::*;

use crate::presentation::views::util::{clamp_box::ClampBox, navigation_page::NavigationPage};

#[relm4::widget_template(pub)]
impl WidgetTemplate for ClampBoxNavigationPage {
    view! {
        #[template]
        #[name = "nav_page"]
        NavigationPage {
            #[template_child]
            contents {
                #[template]
                #[name = "cb"]
                ClampBox,
            }
        }
    }
}

impl AsRef<adw::NavigationPage> for ClampBoxNavigationPage {
    fn as_ref(&self) -> &adw::NavigationPage {
        &self.nav_page
    }
}
