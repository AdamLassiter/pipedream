use ratatui::{style::Stylize, text::Line, widgets::block::Title};

pub fn instructions() -> Title<'static> {
    Title::from(Line::from(vec![
        " Up ".into(),
        "<W>".blue().bold(),
        " Down ".into(),
        "<S>".blue().bold(),
        " Left ".into(),
        "<A>".blue().bold(),
        " Right ".into(),
        "<D>".blue().bold(),
        " Prev ".into(),
        "<Q>".blue().bold(),
        " Next ".into(),
        "<E>".blue().bold(),
        " Enter ".into(),
        "<X>".blue().bold(),
        " Quit ".into(),
        "<Esc> ".blue().bold(),
    ]))
}
