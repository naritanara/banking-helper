use adw::prelude::*;
use relm4::prelude::*;

use crate::presentation::{
    icon_names,
    views::{
        setup::SetupViewPage,
        util::{clamp_box::ClampBox, navigation_page::NavigationPage},
    },
};

pub struct FailedView;

#[derive(Debug)]
pub enum FailedViewInput {
    SetErrorDetails(String),
}

#[relm4::component(pub)]
impl Component for FailedView {
    type Init = ();

    type Input = FailedViewInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[template]
        NavigationPage {
            set_title: "Connection Failed",
            set_tag: Some(SetupViewPage::ConnectionFailed.tag()),

            #[template_child]
            contents {
                adw::StatusPage {
                    set_title: "Something went wrong",
                    set_description: Some("Please check your credentials and try again."),
                    set_icon_name: Some(icon_names::SENTIMENT_DISSATISFIED),

                    #[template]
                    ClampBox {
                        #[template_child]
                        contents {
                            gtk::Expander {
                                set_label: Some("Error details"),
                                gtk::Frame {
                                    #[name = "error_details"]
                                    gtk::TextView {
                                        set_editable: false,
                                        set_monospace: true,
                                    }
                                }
                            },

                            gtk::Button {
                                set_label: "Go back",
                                set_css_classes: &["pill", "suggested-action"],
                                set_action_name: Some("navigation.pop"),
                            }
                        }
                    }
                },
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

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            FailedViewInput::SetErrorDetails(details) => {
                widgets.error_details.buffer().set_text(&details);
            }
        }

        self.update_view(widgets, sender);
    }
}
