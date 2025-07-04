use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, palette::tailwind::SLATE},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};
use std::{ffi::OsString, path::PathBuf};

pub struct App {
    pub current_path: PathBuf,
    pub directories: Vec<OsString>,
    pub files: Vec<OsString>,
    pub exit: bool,
}

impl App {
    pub fn from(current_path: OsString, directories: Vec<OsString>, files: Vec<OsString>) -> Self {
        let current_dir = std::env::current_dir().unwrap();
        Self {
            current_path: current_dir.join(current_path),
            directories,
            files,
            exit: false,
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
            _ => {}
        }
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
            format!("{}", self.current_path.to_string_lossy()),
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

        let combined_list = List::new(all_items).block(
            Block::new()
                .borders(Borders::NONE)
                .border_style(Style::new().fg(SLATE.c400)),
        );

        combined_list.render(main_area, buf);

        Paragraph::new("Press 'q' or 'Esc' to exit").render(footer_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_from() {
        let dirs: Vec<OsString> = Vec::new();
        let f: Vec<OsString> = Vec::new();
        let path = OsString::from("test");
        let app = App::from(path.clone(), dirs.clone(), f.clone());
        assert_eq!(app.directories, dirs);
        assert_eq!(app.files, f);
        assert_eq!(app.current_path, path);
    }
}
