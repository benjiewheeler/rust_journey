use anyhow::{anyhow, Result};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, List, ListDirection, ListState, Paragraph, StatefulWidget, Widget},
    DefaultTerminal, Frame,
};
use std::{env, fs, path::PathBuf};
use toml::{Table, Value};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let app_result = ThemeChanger::default().run(&mut terminal);
    ratatui::restore();

    app_result
}

#[derive(Default, Debug)]
pub struct ThemeChanger {
    config_path: PathBuf,   // Path to the config file.
    config_table: Table,    // Table containing the config file contents.
    themes: Vec<PathBuf>,   // List of themes found in the themes directory.
    input: String,          // The value of the search input field.
    character_index: usize, // The index of the cursor in the input field.
    state: ListState,       // The state of the list widget.
    exit: bool,             // Whether the app should exit.
}

impl ThemeChanger {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.config_path = self.find_config()?;
        self.config_table = self.read_config()?;
        self.themes = self.scan_themes()?;

        // select the first theme
        self.state.select_first();
        self.update_theme();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.update_theme();
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(to_insert) => self.enter_char(to_insert),
            KeyCode::Enter => self.exit(false),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Down => self.select_next(),
            KeyCode::Esc => self.exit(true),
            _ => {}
        }
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        if self.character_index == 0 {
            return;
        }

        let current_index = self.character_index;
        let from_left_to_current_index = current_index - 1;

        let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
        let after_char_to_delete = self.input.chars().skip(current_index);

        self.input = before_char_to_delete.chain(after_char_to_delete).collect();
        self.move_cursor_left();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn exit(&mut self, restore_original: bool) {
        self.exit = true;

        if restore_original {
            let _ = fs::write(&self.config_path, self.config_table.to_string());
        }
    }
}

impl ThemeChanger {
    // function adapted from alacritty's source
    // https://github.com/alacritty/alacritty/blob/6fefa78eafa43f13998439cb9eaf15bc0441f004/alacritty/src/config/mod.rs#L378
    fn find_config(&self) -> Result<PathBuf> {
        let file_name = String::from("alacritty.toml");

        let config_path = xdg::BaseDirectories::with_prefix("alacritty")
            .ok()
            .and_then(|xdg| xdg.find_config_file(&file_name))
            .or_else(|| {
                xdg::BaseDirectories::new()
                    .ok()
                    .and_then(|fallback| fallback.find_config_file(&file_name))
            })
            .or_else(|| {
                if let Ok(home) = env::var("HOME") {
                    // Fallback path: $HOME/.config/alacritty/alacritty.toml.
                    let fallback = PathBuf::from(&home)
                        .join(".config/alacritty")
                        .join(&file_name);
                    if fallback.exists() {
                        return Some(fallback);
                    }
                    // Fallback path: $HOME/.alacritty.toml.
                    let hidden_name = format!(".{file_name}");
                    let fallback = PathBuf::from(&home).join(hidden_name);
                    if fallback.exists() {
                        return Some(fallback);
                    }
                }
                None
            });

        if let None = config_path {
            return Err(anyhow!("Failed to find config file"));
        }

        Ok(config_path.unwrap())
    }

    fn read_config(&mut self) -> Result<Table> {
        let contents = fs::read_to_string(&self.config_path)?;
        let config: Table = contents.parse()?;

        return Ok(config);
    }

    fn scan_themes(&self) -> Result<Vec<PathBuf>> {
        let themes_dir = self.config_path.parent().unwrap().join("themes/themes");
        let files = fs::read_dir(themes_dir)?;

        let mut paths = files
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                if let Ok(metadata) = p.metadata() {
                    metadata.is_file()
                } else {
                    false
                }
            })
            .filter(|p| p.to_string_lossy().ends_with(".toml"))
            .collect::<Vec<_>>();

        // sort the entries alphabetically
        paths.sort_by(|a, b| b.cmp(a));

        return Ok(paths);
    }

    fn update_theme(&mut self) {
        // select the first theme if no theme is selected
        if self.state.selected().is_none() {
            self.state.select_first();
        }

        // get the selected theme index and return if no theme is selected
        let index = self.state.selected();
        if index.is_none() {
            return;
        }

        let items = self.get_matched_themes();

        // check if the selected index is out of bounds
        let index = index.unwrap();
        if index >= items.len() {
            return;
        }

        // get the selected theme
        let theme = &items[index];

        // clone to avoid mutating the original
        let mut config_clone = self.config_table.clone();

        // ensure the general section exists
        if !config_clone.contains_key("general") {
            config_clone.insert("general".to_string(), Value::Table(Table::new()));
        }

        // retrieve the general section
        let general = config_clone
            .get_mut("general")
            .expect("[general] does not exist")
            .as_table_mut()
            .expect("[general] is not a table");

        // ensure the import array exists in the general section
        if !general.contains_key("import") {
            general.insert("import".to_string(), Value::Array(vec![]));
        }

        // retrieve the import array from the general section
        let import = general
            .get_mut("import")
            .expect("[import] does not exist")
            .as_array_mut()
            .expect("[import] is not an array");

        // clear the import array (if any) and push the selected theme
        import.clear();
        import.push(Value::String(theme.to_string_lossy().to_string()));

        // write the updated config
        let _ = fs::write(&self.config_path, config_clone.to_string());
    }

    fn get_matched_themes(&self) -> Vec<PathBuf> {
        let mut items: Vec<_> = self
            .themes
            .iter()
            .zip(self.themes.iter())
            .map(|(o, p)| (o, p.file_name().unwrap().to_str().unwrap().to_string()))
            .map(|(o, p)| (o, p.replace(".toml", "")))
            .map(|(o, p)| (o, SkimMatcherV2::default().fuzzy_match(&p, &self.input)))
            .filter(|(_, m)| m.is_some())
            .collect();

        items.sort_by_key(|(_, m)| m.unwrap());

        return items.iter().rev().map(|(s, _)| s.to_path_buf()).collect();
    }
}

impl Widget for &mut ThemeChanger {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]);
        let [input_area, messages_area] = vertical.areas(area);

        let horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [left_area, right_area] = horizontal.areas(messages_area);

        let input = Paragraph::new(self.input.as_str()).block(
            Block::bordered()
                .title("Filter")
                .border_set(border::PLAIN)
                .border_style(Style::default().fg(Color::Yellow)),
        );
        input.render(input_area, buf);

        let items = self.get_matched_themes();
        let items: Vec<_> = items
            .iter()
            .filter_map(|s| s.file_name())
            .filter_map(|s| s.to_str())
            .map(|s| s.replace(".toml", ""))
            .collect();

        let msg = vec![
            "Press ".into(),
            "esc".bold(),
            " to exit, ".into(),
            "Enter".bold(),
            " to apply".into(),
        ];

        let list = List::new(items)
            .block(
                Block::bordered()
                    .title("Themes")
                    .title_bottom(msg)
                    .border_set(border::PLAIN),
            )
            .highlight_style(Style::new().reversed())
            .highlight_symbol("")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, left_area, buf, &mut self.state);

        let line1 = Line::from(vec![
            " Default ".into(),
            " Black ".fg(Color::Black),
            " White ".fg(Color::White),
            " Gray ".fg(Color::Gray),
            " Red ".fg(Color::Red),
            " Green ".fg(Color::Green),
            " Yellow ".fg(Color::Yellow),
            " Blue ".fg(Color::Blue),
            " Magenta ".fg(Color::Magenta),
            " Cyan ".fg(Color::Cyan),
        ]);
        let line2 = Line::from(vec![
            " Default ".into(),
            " Black ".bg(Color::Black),
            " White ".bg(Color::White),
            " Gray ".bg(Color::Gray),
            " Red ".bg(Color::Red),
            " Green ".bg(Color::Green),
            " Yellow ".bg(Color::Yellow),
            " Blue ".bg(Color::Blue),
            " Magenta ".bg(Color::Magenta),
            " Cyan ".bg(Color::Cyan),
        ]);
        let line3 = Line::from(vec![
            " Default ".into(),
            " Black ".fg(Color::Black).reversed(),
            " White ".fg(Color::White).reversed(),
            " Gray ".fg(Color::Gray).reversed(),
            " Red ".fg(Color::Red).reversed(),
            " Green ".fg(Color::Green).reversed(),
            " Yellow ".fg(Color::Yellow).reversed(),
            " Blue ".fg(Color::Blue).reversed(),
            " Magenta ".fg(Color::Magenta).reversed(),
            " Cyan ".fg(Color::Cyan).reversed(),
        ]);

        let block = Block::bordered().title("Preview").border_set(border::PLAIN);

        Paragraph::new(Text::from(vec![line1, line2, line3]))
            .block(block)
            .render(right_area, buf);
    }
}
