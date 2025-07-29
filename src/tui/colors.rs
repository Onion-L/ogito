use ratatui::style::{
    Color, Modifier, Style,
    palette::{material::PURPLE, tailwind::SLATE},
};

pub const SELECTED_STYLE: Style = Style::new()
    .bg(PURPLE.c600)
    .fg(Color::White)
    .add_modifier(Modifier::BOLD);

pub const COLOR_HEADER: Color = SLATE.c100;
pub const COLOR_ENTRY: Color = SLATE.c400;
pub const COLOR_DISABLED: Color = SLATE.c700;
pub const PREVIEW_STYLE: Style = Style::new().fg(COLOR_ENTRY);
