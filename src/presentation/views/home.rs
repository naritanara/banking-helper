use gtk::prelude::*;
use relm4::prelude::*;

use crate::{
    domain::application::SharedApplication,
    presentation::views::{
        setup::{SetupView, SetupViewOutput},
        util::clamp_box::ClampBox,
    },
};

pub struct HomeView {
    application: SharedApplication,
    setup_view: Option<Controller<SetupView>>,
}

#[derive(Debug)]
pub enum HomeInput {
    OpenInitialSetup,
    InitialSetupDone,
}

#[relm4::component(pub)]
impl Component for HomeView {
    type Init = SharedApplication;

    type Input = HomeInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::Window {
            set_title: Some("Banking Helper"),

            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},

                #[template]
                ClampBox {

                    #[template_child]
                    contents {
                        gtk::Label {
                            set_text: "The application has not been set up yet. Click the button below to start the initial setup."
                        },

                        gtk::Button::with_label("Start Initial Setup") {
                            #[watch]
                            set_sensitive: model.setup_view.is_none(),
                            connect_clicked => HomeInput::OpenInitialSetup,
                        }
                    }
                }
            },
        }
    }

    fn init(
        application: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            application,
            setup_view: None,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            HomeInput::OpenInitialSetup => {
                let setup_view = self.setup_view.insert(
                    SetupView::builder()
                        .launch(self.application.clone())
                        .forward(sender.input_sender(), |msg| match msg {
                            SetupViewOutput::Closed => HomeInput::InitialSetupDone,
                        }),
                );
                setup_view.widget().present();
            }
            HomeInput::InitialSetupDone => {
                self.setup_view = None;
            }
        }
    }
}
