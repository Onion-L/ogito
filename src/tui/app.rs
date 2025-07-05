use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        Color, Modifier, Style,
        palette::{material::PURPLE, tailwind::SLATE},
    },
    text::{Line, Span},
    widgets::{
        Block, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget, Widget,
    },
};
use std::{ffi::OsString, path::PathBuf};

const SELECTED_STYLE: Style = Style::new()
    .bg(PURPLE.c600)
    .fg(Color::White)
    .add_modifier(Modifier::BOLD);

pub struct App {
    pub path: PathBuf,
    pub directories: Vec<OsString>,
    pub files: Vec<OsString>,
    pub exit: bool,
    list_state: ListState,
}

impl App {
    pub fn from(current_path: OsString, directories: Vec<OsString>, files: Vec<OsString>) -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let mut list_state = ListState::default();

        if !directories.is_empty() || !files.is_empty() {
            list_state.select(Some(0));
        }

        let path = current_dir.join(current_path);

        Self {
            path,
            directories,
            files,
            exit: false,
            list_state,
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                frame.render_widget(&mut self, frame.area());
            })?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            KeyCode::Down | KeyCode::Char('j') => self.select_next(),
            KeyCode::Up | KeyCode::Char('k') => self.select_previous(),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }

    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        Span::styled(
            format!("{}", self.path.to_string_lossy()),
            Style::new().fg(SLATE.c100),
        )
        .render(header_area, buf);

        let mut all_items: Vec<ListItem> = Vec::new();

        for dir in &self.directories {
            let dir_name = dir.to_string_lossy();
            let line = Line::styled(format!("📁 {}", dir_name), SLATE.c400);
            all_items.push(ListItem::new(line));
        }

        for file in &self.files {
            let file_name = file.to_string_lossy();
            let line = Line::styled(format!("📄 {}", file_name), SLATE.c400);
            all_items.push(ListItem::new(line));
        }

        let combined_list = List::new(all_items)
            .block(Block::new())
            .highlight_style(SELECTED_STYLE)
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(combined_list, main_area, buf, &mut self.list_state);

        Paragraph::new("Press 'q' or 'Esc' to exit").render(footer_area, buf);
    }
}
