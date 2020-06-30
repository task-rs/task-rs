use super::super::components::{Main, MainMessage};
use super::{model::view::tasks::FilterMethod, Message, Model};
use iced::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    Main { model }.view().map(|message| match message {
        MainMessage::SetTaskStatusFilter(filter_method) => {
            Message::SetTaskStatusFilter(filter_method)
        }
        MainMessage::SetDarkMode(dark_mode) => Message::SetDarkMode(dark_mode),
        MainMessage::SetTaskFilterMethodToAll => Message::SetTaskFilterMethod(FilterMethod::All),
        MainMessage::SetTaskFilterMethodToSingleTag => {
            Message::SetTaskFilterMethod(FilterMethod::SingleTag)
        }
        MainMessage::SetTaskFilterMethodToMultipleTags => {
            Message::SetTaskFilterMethod(FilterMethod::MultipleTags)
        }
        MainMessage::FilterTasksBySingleTag(index) => Message::FilterTasksBySingleTag(index),
        MainMessage::AddTagToMultipleTags(index) => Message::AddTagToMultipleTags(index),
        MainMessage::RemoveTagFromMultipleTags(index) => Message::RemoveTagFromMultipleTags(index),
        MainMessage::CheckAllOfMultipleTags => Message::CheckAllOfMultipleTags,
        MainMessage::UncheckAllOfMultipleTags => Message::UncheckAllOfMultipleTags,
        MainMessage::InvertAllOfMultipleTags => Message::InvertAllOfMultipleTags,
        MainMessage::SetTaskStatus(address, status) => Message::SetTaskStatus(address, status),
    })
}
