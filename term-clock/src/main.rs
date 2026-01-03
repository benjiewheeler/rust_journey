use clap::{Arg, command, value_parser};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    prelude::{Frame, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};
use std::{io, time::Duration};
use tui_big_text::{BigText, PixelSize};

#[derive(Default)]
pub struct App {
    exit: bool,

    show_weekday: bool,
    show_date: bool,
    show_time: bool,
    show_seconds: bool,
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        show_weekday: bool,
        show_date: bool,
        show_time: bool,
        show_seconds: bool,
    ) -> io::Result<()> {
        self.show_weekday = show_weekday;
        self.show_date = show_date;
        self.show_time = show_time;
        self.show_seconds = show_seconds;

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        // Poll with timeout to refresh every second
        if event::poll(Duration::from_millis(1000))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().title_bottom(
            Line::from("Press 'q' to exit")
                .style(Style::new().dark_gray())
                .alignment(Alignment::Center),
        );
        let inner = block.inner(area);
        block.render(area, buf);

        let now = chrono::Local::now();

        let weekday_size = if self.show_weekday { 2 + 1 } else { 0 }; // add 1 for spacing
        let date_size = if self.show_date { 2 + 1 } else { 0 }; // add 1 for spacing
        let time_size = if self.show_time { 8 } else { 0 };

        // create a layout centered vertically and horizontally
        let vertical_layout =
            Layout::vertical([Constraint::Length(weekday_size + date_size + time_size)])
                .flex(Flex::Center)
                .split(inner);

        // create 3 sections for day, date and time
        let sections = Layout::vertical([
            Constraint::Length(weekday_size),
            Constraint::Length(date_size),
            Constraint::Length(time_size),
        ])
        .split(vertical_layout[0]);

        if self.show_weekday {
            let day_line = Line::from(Span::styled(
                now.format("%A").to_string(),
                Style::new().red(),
            ));

            BigText::builder()
                .pixel_size(PixelSize::Octant)
                .lines(vec![day_line])
                .centered()
                .build()
                .render(sections[0], buf);
        }

        if self.show_date {
            let date_line = Line::from(Span::styled(
                now.format("%b %d, %Y").to_string(),
                Style::new().yellow(),
            ));

            BigText::builder()
                .pixel_size(PixelSize::Octant)
                .lines(vec![date_line])
                .centered()
                .build()
                .render(sections[1], buf);
        }

        if self.show_time {
            let format = if self.show_seconds {
                "%H:%M:%S"
            } else {
                "%H:%M"
            };

            let time_line = Line::from(Span::styled(
                now.format(format).to_string(),
                Style::new().blue(),
            ));

            BigText::builder()
                .pixel_size(PixelSize::Full)
                .lines(vec![time_line])
                .centered()
                .build()
                .render(sections[2], buf);
        }
    }
}

fn main() -> io::Result<()> {
    // parse the command line arguments
    let matches = command!()
        .name("term-clock")
        .about("A simple digital terminal clock")
        .arg(
            Arg::new("show-weekday")
                .long("show-weekday")
                .short('w')
                .num_args(0)
                .default_value("false")
                .value_parser(value_parser!(bool))
                .help("Show the weekday name"),
        )
        .arg(
            Arg::new("show-date")
                .long("show-date")
                .short('d')
                .num_args(0)
                .default_value("false")
                .value_parser(value_parser!(bool))
                .help("Show the date"),
        )
        .arg(
            Arg::new("show-time")
                .long("show-time")
                .short('t')
                .num_args(0)
                .default_value("false")
                .value_parser(value_parser!(bool))
                .help("Show the time"),
        )
        .arg(
            Arg::new("show-seconds")
                .long("show-seconds")
                .short('s')
                .num_args(0)
                .default_value("false")
                .value_parser(value_parser!(bool))
                .help("Show seconds"),
        )
        .get_matches();

    let mut show_weekday = matches.get_one::<bool>("show-weekday").unwrap().clone();
    let mut show_date = matches.get_one::<bool>("show-date").unwrap().clone();
    let mut show_time = matches.get_one::<bool>("show-time").unwrap().clone();
    let show_seconds = matches.get_one::<bool>("show-seconds").unwrap().clone();

    // default case if all args are false, show everything
    if !show_weekday && !show_date && !show_time {
        show_weekday = true;
        show_date = true;
        show_time = true;
    }

    ratatui::run(|terminal| {
        App::default().run(terminal, show_weekday, show_date, show_time, show_seconds)
    })
}
