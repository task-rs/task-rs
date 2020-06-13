use super::super::super::mvc::UiState;
use super::App;
use iced::Settings;

impl App {
    pub(crate) fn settings(&self) -> Settings<UiState> {
        let mut settings = Settings::with_flags(self.flags());
        settings.antialiasing = true;
        settings
    }
}
