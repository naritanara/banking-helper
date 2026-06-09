use std::path::Path;

use adw::prelude::*;
use relm4::prelude::*;
use thiserror::Error;

use crate::{
    domain::{
        application::SharedApplication,
        enable_banking::{
            api_credentials::{ApiCredentials, TestCredentialsError},
            application_id::ApplicationId,
            private_key::{PrivateKey, PrivateKeyLoadError},
        },
    },
    presentation::views::setup::{
        failed::{FailedView, FailedViewInput},
        settings::{SettingsView, SettingsViewOutput},
        success::{SuccessView, SuccessViewOutput},
        testing::TestingView,
    },
};

mod failed;
mod settings;
mod success;
mod testing;

#[derive(Debug, Clone, Copy)]
pub enum SetupViewPage {
    ApiSettings,
    TestConnection,
    ConnectionFailed,
    Success,
}

impl SetupViewPage {
    const fn tag(self) -> &'static str {
        match self {
            SetupViewPage::ApiSettings => "api-settings",
            SetupViewPage::TestConnection => "test-connection",
            SetupViewPage::ConnectionFailed => "connection-failed",
            SetupViewPage::Success => "success",
        }
    }
}

pub struct SetupView {
    application: SharedApplication,
    settings_page: Controller<SettingsView>,
    testing_page: Controller<TestingView>,
    failed_page: Controller<FailedView>,
    success_page: Controller<SuccessView>,
}

#[derive(Debug)]
pub enum SetupViewInput {
    SettingsPageDone(SettingsViewOutput),
    Finish,
}

#[derive(Debug)]
pub enum SetupViewCommandOutput {
    SetApiCredentials(Result<(), SetApiCredentialsError>),
}

#[derive(Debug)]
pub enum SetupViewOutput {
    Closed,
}

#[relm4::component(pub)]
impl Component for SetupView {
    type Init = SharedApplication;

    type Input = SetupViewInput;
    type Output = SetupViewOutput;
    type CommandOutput = SetupViewCommandOutput;

    view! {
        adw::Window {
            set_title: Some("Banking Helper Setup"),

            connect_close_request[sender] => move |_| {
                sender.output_sender().emit(SetupViewOutput::Closed);
                gtk::glib::Propagation::Proceed
            },

            #[name = "navigation_view"]
            adw::NavigationView {
                model.settings_page.widget(),
                model.testing_page.widget(),
                model.failed_page.widget(),
                model.success_page.widget(),
            }
        }
    }

    fn init(
        application: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let pem_key_filter = gtk::FileFilter::new();
        pem_key_filter.add_suffix("pem");

        let model = Self {
            application: application.clone(),
            settings_page: SettingsView::builder()
                .launch(application)
                .forward(sender.input_sender(), SetupViewInput::SettingsPageDone),
            testing_page: TestingView::builder().launch(()).detach(),
            failed_page: FailedView::builder().launch(()).detach(),
            success_page: SuccessView::builder()
                .launch(())
                .forward(sender.input_sender(), |SuccessViewOutput::FinishRequest| {
                    SetupViewInput::Finish
                }),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            SetupViewInput::SettingsPageDone(output) => {
                let application = self.application.clone();

                sender.command(async move |sender, _| {
                    sender.emit(SetupViewCommandOutput::SetApiCredentials(
                        Self::set_api_credentials(
                            application,
                            output.application_id,
                            output.private_key_path,
                        )
                        .await,
                    ))
                });
            }
            SetupViewInput::Finish => {
                root.close();
            }
        }
    }

    fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::CommandOutput,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            SetupViewCommandOutput::SetApiCredentials(Ok(())) => {
                widgets
                    .navigation_view
                    .push_by_tag(SetupViewPage::Success.tag());
            }
            SetupViewCommandOutput::SetApiCredentials(Err(e)) => {
                let mut e: &dyn std::error::Error = &e;
                let mut details = e.to_string();
                while let Some(source) = e.source() {
                    e = source;
                    details.push_str(&format!("\nCaused by: {e}"));
                }

                self.failed_page
                    .sender()
                    .emit(FailedViewInput::SetErrorDetails(details));
                widgets
                    .navigation_view
                    .push_by_tag(SetupViewPage::ConnectionFailed.tag());
                widgets.navigation_view.replace_with_tags(&[
                    SetupViewPage::ApiSettings.tag(),
                    SetupViewPage::ConnectionFailed.tag(),
                ]);
            }
        }

        self.update_view(widgets, sender);
    }
}

impl SetupView {
    pub async fn set_api_credentials(
        application: SharedApplication,
        application_id: String,
        private_key_path: impl AsRef<Path>,
    ) -> Result<(), SetApiCredentialsError> {
        let application_id = ApplicationId::new(application_id);
        let private_key = PrivateKey::load_from_file(private_key_path).await?;
        let api_credentials = ApiCredentials::new(application_id, private_key);

        let mut app = application.write().await;

        let api_credentials = app.set_api_credentials(api_credentials);

        if let Err(e) = api_credentials.test_credentials().await {
            app.remove_api_credentials();
            return Err(e.into());
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum SetApiCredentialsError {
    #[error("error loading private key")]
    PrivateKeyLoadError(#[from] PrivateKeyLoadError),
    #[error("error testing credentials against the API")]
    TestCredentialsError(#[from] TestCredentialsError),
}
