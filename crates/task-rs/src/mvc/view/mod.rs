use super::{Message, Model};
use iced::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    let style = model.ui_state.theme.style();

    let sidebar = {
        let mut sidebar = Column::new().push(Text::new("TaskRs").size(40));

        for (tag_id, tag_data) in &model.data.tags {
            sidebar = sidebar.push(Text::new(
                tag_data.name.clone().unwrap_or_else(|| tag_id.0.clone()),
            ));
        }

        sidebar
    };

    let row = Row::new().push(sidebar);

    Container::new(row)
        .style(stylesheets::Container(style))
        .into()
}

mod stylesheets;
