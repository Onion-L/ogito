use crate::file::{Repo, get_repo};
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
        Block, HighlightSpacing, List, ListItem, ListState, Paragraph, ScrollbarState,
        StatefulWidget, Widget, Wrap,
    },
};
use std::{ffi::OsString, fs, path::PathBuf};

// TODO make the preview section scrollable
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    FileList,
    Preview,
}

const SELECTED_STYLE: Style = Style::new()
    .bg(PURPLE.c600)
    .fg(Color::White)
    .add_modifier(Modifier::BOLD);

pub struct App {
    pub path: PathBuf,
    pub repo: Repo,
    pub list_state: ListState,
    pub file_content: String,
    pub show_preview: bool,
    pub exit: bool,
    pub root: PathBuf,
    pub focus: Focus,
    pub preview_scroll_offset: usize,
    pub preview_lines: Vec<String>,
    pub scrollbar_state: ScrollbarState,
}

impl App {
    pub fn from(current_path: OsString, repo: Repo) -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let mut list_state = ListState::default();

        if !repo.directories.is_empty() || !repo.files.is_empty() {
            list_state.select(Some(0));
        }

        let path = current_dir.join(current_path);
        let path = fs::canonicalize(&path).unwrap_or(path);
        let root = path.clone();
        Self {
            path,
            root,
            repo,
            exit: false,
            list_state,
            file_content: String::new(),
            show_preview: false,
            focus: Focus::FileList,
            preview_scroll_offset: 0,
            preview_lines: Vec::new(),
            scrollbar_state: ScrollbarState::default(),
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
            KeyCode::Enter => self.handle_enter(),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }

    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    fn is_file_selected(&self, selected: usize) -> bool {
        selected >= self.repo.directories.len()
    }

    fn get_canonical_path(&self, current_path: &OsString) -> PathBuf {
        let path = self.path.join(current_path);
        std::fs::canonicalize(&path).unwrap_or_else(|_| path.clone())
    }

    fn add_parent_directory_if_needed(&self, repo: &mut Repo, path: &PathBuf) {
        if *path != self.root {
            repo.directories.insert(0, "..".into());
        }
    }

    fn handle_enter(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if self.is_file_selected(selected) {
                self.handle_file_selection(selected);
            } else {
                self.handle_dir_selection(selected);
            }
        }
    }

    fn handle_dir_selection(&mut self, selected: usize) {
        let current_dir = &self.repo.directories[selected];
        let path = self.get_canonical_path(current_dir);
        let mut repo = get_repo(&OsString::from(&path)).unwrap();
        self.add_parent_directory_if_needed(&mut repo, &path);
        self.repo = repo;
        self.path = path.clone();
        self.show_preview = false;
    }

    fn handle_file_selection(&mut self, selected: usize) {
        self.show_preview = true;
        let file_index = selected - self.repo.directories.len();
        let file_name = &self.repo.files[file_index];
        let file_path = self.get_canonical_path(file_name);
        // TODO more file types
        match fs::read_to_string(file_path) {
            Ok(content) => self.file_content = content,
            Err(e) => self.file_content = format!("Error reading file: {}", e),
        };
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
            &self.path.to_string_lossy()[4..],
            Style::new().fg(SLATE.c100),
        )
        .render(header_area, buf);

        let (repo_area, gap_area, preview_area) = if self.show_preview {
            let chunks = Layout::horizontal([
                Constraint::Percentage(40),
                Constraint::Percentage(1),
                Constraint::Percentage(59),
            ])
            .split(main_area);
            (chunks[0], Some(chunks[1]), Some(chunks[2]))
        } else {
            (main_area, None, None)
        };
        let mut all_items: Vec<ListItem> = Vec::new();

        for dir in &self.repo.directories {
            let dir_name = dir.to_string_lossy();
            let line = Line::styled(format!("📁 {}", dir_name), SLATE.c400);
            all_items.push(ListItem::new(line));
        }

        for file in &self.repo.files {
            let file_name = file.to_string_lossy();
            let line = Line::styled(format!("📄 {}", file_name), SLATE.c400);
            all_items.push(ListItem::new(line));
        }

        let combined_list = List::new(all_items)
            .block(Block::new())
            .highlight_style(SELECTED_STYLE)
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(combined_list, repo_area, buf, &mut self.list_state);

        if let Some(gap) = gap_area {
            Block::new().render(gap, buf);
        }

        let preview_content = Paragraph::new(self.file_content.clone())
            .block(Block::new())
            .wrap(Wrap { trim: false })
            .style(Style::new().fg(SLATE.c400));

        if let Some(preview) = preview_area {
            preview_content.render(preview, buf);
        }
        Paragraph::new("Press 'q' or 'Esc' to exit").render(footer_area, buf);
    }
}
