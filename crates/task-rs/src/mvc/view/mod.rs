use super::super::data::tag;
use super::{Message, Model};
use iced::*;
use pipe_trait::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    let style = model.ui_state.theme.style();

    let sidebar = {
        let mut sidebar = Column::new().push(Text::new("TaskRs").size(40));

        for entry in &model.data.tags {
            sidebar = entry
                .pipe(tag::entry::display)
                .pipe(Text::new)
                .pipe(|text| sidebar.push(text))
        }

        sidebar
    };

    let row = Row::new().push(sidebar);

    Container::new(row)
        .style(stylesheets::Container(style))
        .into()
}

mod stylesheets;
