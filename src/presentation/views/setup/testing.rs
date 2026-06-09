use adw::prelude::*;
use relm4::prelude::*;

use crate::presentation::views::{setup::SetupViewPage, util::navigation_page::NavigationPage};

pub struct TestingView;

#[relm4::component(pub)]
impl SimpleComponent for TestingView {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        #[template]
        NavigationPage {
            set_title: "Connecting...",
            set_tag: Some(SetupViewPage::TestConnection.tag()),

            #[template_child]
            contents {
                #[name = "status_page"]
                adw::StatusPage {
                    set_title: "Connecting to Enable Banking...",
                    set_description: Some("Your credentials are being checked for validity."),
                    #[wrap(Some)]
                    set_paintable = &adw::SpinnerPaintable::new(Some(&status_page)),
                }
            }
        },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
