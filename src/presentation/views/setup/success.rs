use adw::prelude::*;
use relm4::prelude::*;

use crate::presentation::{
    icon_names,
    views::{
        setup::SetupViewPage,
        util::{clamp_box::ClampBox, navigation_page::NavigationPage},
    },
};

pub struct SuccessView;

#[derive(Debug)]
pub enum SuccessViewInput {
    FinishClicked,
}

#[derive(Debug)]
pub enum SuccessViewOutput {
    FinishRequest,
}

#[relm4::component(pub)]
impl SimpleComponent for SuccessView {
    type Init = ();
    type Input = SuccessViewInput;
    type Output = SuccessViewOutput;

    view! {
        #[template]
        NavigationPage {
            set_title: "Done",
            set_tag: Some(SetupViewPage::Success.tag()),
            set_can_pop: false,

            #[template_child]
            contents {
                adw::StatusPage {
                    set_title: "Success",
                    set_description: Some("You're all set!"),
                    set_icon_name: Some(icon_names::VERIFIED_CHECKMARK),

                    #[template]
                    ClampBox {
                        gtk::Button {
                            set_label: "Finish setup",
                            set_css_classes: &["pill", "suggested-action"],
                            connect_clicked => SuccessViewInput::FinishClicked,
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SuccessViewInput::FinishClicked => sender
                .output_sender()
                .emit(SuccessViewOutput::FinishRequest),
        }
    }
}
