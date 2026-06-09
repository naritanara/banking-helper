use std::path::PathBuf;

use adw::prelude::*;
use relm4::prelude::*;
use relm4_components::open_dialog::{
    OpenDialog, OpenDialogResponse, OpenDialogSettings, SingleSelection,
};

use crate::{domain::application::SharedApplication, presentation::{
    icon_names,
    views::{
        setup::SetupViewPage,
        util::{
            boxed_list::BoxedList, clamp_box_navigation_page::ClampBoxNavigationPage,
            navigation_button::NavigationButton,
        },
    },
}};

#[derive(Debug)]
pub enum SettingsViewInput {
    PrivateKeyPathChooserRequested,
    PrivateKeyPathChooserResponse(OpenDialogResponse<SingleSelection>),
    RecomputeCanContinue,
    ContinueRequested,
}

#[derive(Debug)]
pub struct SettingsViewOutput {
    pub application_id: String,
    pub private_key_path: PathBuf,
}

pub struct SettingsView {
    private_key_path_chooser_dialog: Controller<OpenDialog>,
    can_continue: bool,
}

#[relm4::component(pub)]
impl Component for SettingsView {
    type Init = SharedApplication;

    type Input = SettingsViewInput;
    type Output = SettingsViewOutput;
    type CommandOutput = ();

    view! {
        #[template]
        ClampBoxNavigationPage {
            set_title: "API Settings",
            set_tag: Some(SetupViewPage::ApiSettings.tag()),

            #[template_child]
            cb.contents {
                #[template]
                BoxedList {
                    #[name = "application_id_entry"]
                    adw::EntryRow {
                        set_title: "Application ID",
                        add_prefix = &gtk::Image::from_icon_name(icon_names::PUZZLE_PIECE),

                        connect_changed => SettingsViewInput::RecomputeCanContinue,
                    },
                },

                #[template]
                BoxedList {
                    #[name = "private_key_path_entry"]
                    adw::EntryRow {
                        set_title: "Path to private key file",
                        set_editable: false,
                        add_prefix = &gtk::Image::from_icon_name(icon_names::KEY4),
                        connect_changed => SettingsViewInput::RecomputeCanContinue,
                    },

                    adw::ButtonRow {
                        set_title: "Browse...",
                        connect_activated => SettingsViewInput::PrivateKeyPathChooserRequested,
                    },
                },

                #[template]
                NavigationButton("Continue") {
                    connect_clicked => SettingsViewInput::ContinueRequested,

                    #[watch]
                    set_navigation_target: model.can_continue.then_some(SetupViewPage::TestConnection.tag()),
                }
            }
        },
    }

    fn init(
        application: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let pem_key_filter = gtk::FileFilter::new();
        pem_key_filter.add_suffix("pem");

        let model = Self {
            private_key_path_chooser_dialog: OpenDialog::builder()
                .launch(OpenDialogSettings {
                    is_modal: true,
                    accept_label: "Select".into(),
                    filters: vec![pem_key_filter],
                    ..Default::default()
                })
                .forward(
                    sender.input_sender(),
                    SettingsViewInput::PrivateKeyPathChooserResponse,
                ),
            can_continue: false,
        };

        let widgets = view_output!(model);

        if let Some(credentials) = application.blocking_read().get_api_credentials() {
            widgets
                .application_id_entry
                .set_text(credentials.application_id());
        }

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
            SettingsViewInput::PrivateKeyPathChooserRequested => {
                self.private_key_path_chooser_dialog.widget().show()
            }
            SettingsViewInput::PrivateKeyPathChooserResponse(OpenDialogResponse::Accept(p)) => {
                widgets
                    .private_key_path_entry
                    .set_text(&p.to_string_lossy());
            }
            SettingsViewInput::PrivateKeyPathChooserResponse(OpenDialogResponse::Cancel) => (), // Do nothing
            SettingsViewInput::RecomputeCanContinue => {
                let application_id = widgets.application_id_entry.text().to_string();
                let private_key_path =
                    PathBuf::from(widgets.private_key_path_entry.text().as_str());
                self.can_continue = !application_id.is_empty() && private_key_path.is_file();
            }
            SettingsViewInput::ContinueRequested => {
                sender.output_sender().emit(SettingsViewOutput {
                    application_id: widgets.application_id_entry.text().to_string(),
                    private_key_path: PathBuf::from(widgets.private_key_path_entry.text()),
                });
            }
        }

        self.update_view(widgets, sender);
    }
}
