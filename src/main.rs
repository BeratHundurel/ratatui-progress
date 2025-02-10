use std::io;

use ratatui::{
    crossterm::{
        self,
        event::{KeyCode, KeyEventKind},
    },
    layout::{Constraint, Layout, Rect},
    style::{self, Color, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Gauge, Widget},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App {
        exit: false,
        progress_bar_color: Color::Green,
    };

    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}

pub struct App {
    exit: bool,
    progress_bar_color: Color,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }

            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true;
        } else if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('c') {
            self.progress_bar_color = match self.progress_bar_color {
                Color::Green => Color::Red,
                Color::Red => Color::Blue,
                Color::Blue => Color::Green,
                _ => Color::Green,
            };
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::buffer::Buffer)
    where
        Self: Sized,
    {
        let vertical_layout =
            Layout::vertical(&[Constraint::Percentage(20), Constraint::Percentage(80)]).margin(1);

        let [title_area, gauge_area] = vertical_layout.areas(area);

        Line::from("Process overview")
            .bold()
            .style(style::Style::default().fg(Color::Yellow))
            .render(title_area, buf);

        let instructions = Line::from(vec![
            " Change color ".into(),
            "<C>".blue().bold(),
            " | Quit ".into(),
            "<Q>".red().bold(),
        ]);

        let block = Block::bordered()
            .title(Line::from("Background processes"))
            .title_bottom(instructions)
            .border_set(border::THICK);

        let progress_bar = Gauge::default()
            .gauge_style(Style::default().fg(self.progress_bar_color))
            .block(block)
            .fg(Color::LightBlue)
            .label(Span::styled(
                "Process 1: 50%",
                Style::default().fg(Color::DarkGray),
            ))
            .ratio(0.5);

        progress_bar.render(
            Rect {
                x: gauge_area.left(),
                y: gauge_area.top(),
                width: gauge_area.width,
                height: 3,
            },
            buf,
        );
    }
}
